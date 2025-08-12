use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, BufRead, Write};
use std::time::{Duration, Instant};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sysinfo::{System};
use ragit_utils::memory_utils::print_memory_usage;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TermEmbeddings(HashMap<String, Vec<f64>>);

#[derive(Debug, Clone)]
struct Node {
    path: String,
    embedding: Vec<f64>,
    position: Vec<f64>,
    velocity: Vec<f64>,
}

fn calculate_energy(nodes: &[Node]) -> f64 {
    let mut energy = 0.0;
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let dist_sq = nodes[i].position.iter().zip(nodes[j].position.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>();
            let path_similarity = cosine_similarity(&nodes[i].embedding, &nodes[j].embedding);
            energy += (1.0 - path_similarity) * dist_sq.sqrt();
        }
    }
    energy
}

fn calculate_energy_delta(nodes: &[Node], node_index: usize, original_position: &[f64]) -> f64 {
    let mut delta = 0.0;
    let node = &nodes[node_index];

    for (i, other_node) in nodes.iter().enumerate() {
        if i == node_index {
            continue;
        }

        let old_dist_sq = original_position.iter().zip(other_node.position.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>();
        let new_dist_sq = node.position.iter().zip(other_node.position.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>();
        let path_similarity = cosine_similarity(&node.embedding, &other_node.embedding);

        delta += (1.0 - path_similarity) * (new_dist_sq.sqrt() - old_dist_sq.sqrt());
    }

    delta
}

fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(&a, &b)| a * b).sum();
    let magnitude1: f64 = vec1.iter().map(|&a| a * a).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.iter().map(|&a| a * a).sum::<f64>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}

fn run_simulated_annealing(
    nodes: &mut Vec<Node>,
    initial_temp: f64,
    cooling_rate: f64,
    iterations: usize,
    viscosity: f64,
    sys: &mut System,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
    start_time: Instant,
    timeout: Duration,
) {
    print_memory_usage(sys, "Inside run_simulated_annealing...", last_snapshot_data);
    let mut rng = rand::thread_rng();
    print_memory_usage(sys, "Calculating initial energy...", last_snapshot_data);
    let mut current_energy = calculate_energy(nodes);
    print_memory_usage(sys, &format!("Initial energy: {:.4}", current_energy), last_snapshot_data);
    let mut temp = initial_temp;

    for i in 0..iterations {
        if start_time.elapsed() > timeout {
            println!("Timeout reached. Stopping simulation.");
            io::stdout().flush().unwrap();
            break;
        }

        let node_index = rng.gen_range(0..nodes.len());
        let original_position = nodes[node_index].position.clone();

        // Propose a new position
        for dim in 0..nodes[node_index].position.len() {
            nodes[node_index].position[dim] += (rng.r#gen::<f64>() - 0.5) * temp;
        }

        let delta_energy = calculate_energy_delta(nodes, node_index, &original_position);

        if delta_energy < 0.0 || rng.r#gen::<f64>() < (-delta_energy / temp).exp() {
            current_energy += delta_energy;
        } else {
            nodes[node_index].position = original_position;
        }

        // Update positions based on velocity and viscosity
        for node in nodes.iter_mut() {
            for dim in 0..node.position.len() {
                node.position[dim] += node.velocity[dim];
                node.velocity[dim] *= 1.0 - viscosity;
            }
        }

        temp *= cooling_rate;
        if i % 100 == 0 {
            print_memory_usage(sys, &format!("Iteration {}: Temp = {:.4}, Energy = {:.4}", i, temp, current_energy), last_snapshot_data);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    let mut last_snapshot_data = None;
    print_memory_usage(&mut sys, "Starting chunk comparison program...", &mut last_snapshot_data);

    let start_time = Instant::now();
    let timeout = Duration::from_secs(300);

    let term_embeddings_path = "term_embeddings.json";
    let index_file_path = "index.rs.txt";
    let embedding_dimension = 8;

    print_memory_usage(&mut sys, "Reading term embeddings...", &mut last_snapshot_data);
    let embeddings_content = fs::read_to_string(term_embeddings_path)?;
    let term_embeddings: TermEmbeddings = serde_json::from_str(&embeddings_content)?;
    let term_map = term_embeddings.0;
    print_memory_usage(&mut sys, "Finished reading term embeddings.", &mut last_snapshot_data);

    print_memory_usage(&mut sys, "Reading index file...", &mut last_snapshot_data);
    let index_file = fs::File::open(index_file_path)?;
    let reader = BufReader::new(index_file);
    let file_paths: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    print_memory_usage(&mut sys, "Finished reading index file.", &mut last_snapshot_data);

    print_memory_usage(&mut sys, "Creating nodes...", &mut last_snapshot_data);
    let mut nodes = Vec::new();
    let mut rng = rand::thread_rng();

    for path in file_paths {
        let mut embedding = vec![0.0; embedding_dimension];
        let path_parts: Vec<&str> = path.split('/').collect();
        let num_parts = path_parts.len();

        for (i, part) in path_parts.iter().enumerate() {
            let weight = if i == num_parts - 1 { 3.0 } else { 1.0 };
            for term in part.split(|c: char| !c.is_alphanumeric()) {
                if let Some(emb) = term_map.get(&term.to_lowercase()) {
                    for d in 0..embedding_dimension {
                        embedding[d] += emb[d] * weight;
                    }
                }
            }
        }

        let position = (0..embedding_dimension).map(|_| rng.r#gen::<f64>()).collect();
        let velocity = (0..embedding_dimension).map(|_| 0.0).collect();

        nodes.push(Node { path, embedding, position, velocity });
    }
    print_memory_usage(&mut sys, "Finished creating nodes.", &mut last_snapshot_data);

    print_memory_usage(&mut sys, "Starting simulated annealing...", &mut last_snapshot_data);
    run_simulated_annealing(&mut nodes, 1.0, 0.99, 1000, 0.01, &mut sys, &mut last_snapshot_data, start_time, timeout);
    print_memory_usage(&mut sys, "Finished simulated annealing.", &mut last_snapshot_data);

    // Find and report similar pairs
    let mut similar_pairs = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let dist_sq = nodes[i].position.iter().zip(nodes[j].position.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>();
            if dist_sq.sqrt() < 0.1 {
                similar_pairs.push((nodes[i].path.clone(), nodes[j].path.clone(), dist_sq.sqrt()));
            }
        }
    }

    if similar_pairs.is_empty() {
        println!("No very similar file paths found");
    } else {
        println!("Found {} similar file paths:", similar_pairs.len());
        for (path1, path2, distance) in similar_pairs {
            println!("  - Path 1: {}", path1);
            println!("    Path 2: {}", path2);
            println!("    Distance: {:.4}", distance);
            println!("--------------------------------------------------");
        }
    }

    print_memory_usage(&mut sys, "Final", &mut last_snapshot_data);

    Ok(())
}
