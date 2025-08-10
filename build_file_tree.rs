use std::collections::HashMap;
use serde::Serialize;
use ragit_feature_extractor::FileReport;

#[derive(Debug, Serialize)]
struct TreeNode {
    name: String,
    term_counts: HashMap<String, u32>,
    total_weight: f64,
    fraction: f64,
    children: HashMap<String, TreeNode>,
}

impl TreeNode {
    fn new(name: &str) -> Self {
        TreeNode {
            name: name.to_string(),
            term_counts: HashMap::new(),
            total_weight: 0.0,
            fraction: 0.0,
            children: HashMap::new(),
        }
    }

    fn add_path(&mut self, path_parts: &[&str], file_report: &FileReport) {
        if path_parts.is_empty() {
            // This is the end of the path, add file's term counts here
            for (term, count) in &file_report.term_counts {
                *self.term_counts.entry(term.clone()).or_insert(0) += count;
            }
            return;
        }

        let (current_part, remaining_parts) = path_parts.split_first().unwrap();
        let child = self.children.entry(current_part.to_string()).or_insert_with(|| TreeNode::new(current_part));
        child.add_path(remaining_parts, file_report);
    }

    fn aggregate_term_counts(&mut self) -> HashMap<String, u32> {
        let mut aggregated_counts = self.term_counts.clone();

        for child in self.children.values_mut() {
            let child_counts = child.aggregate_term_counts();
            for (term, count) in child_counts {
                *aggregated_counts.entry(term).or_insert(0) += count;
            }
        }

        self.term_counts = aggregated_counts.clone();
        aggregated_counts
    }

    fn calculate_weights_and_fractions(&mut self, parent_total_weight: f64) {
        self.total_weight = self.term_counts.values().sum::<u32>() as f64;
        if parent_total_weight > 0.0 {
            self.fraction = self.total_weight / parent_total_weight;
        }

        let current_total_weight = self.total_weight;
        for child in self.children.values_mut() {
            child.calculate_weights_and_fractions(current_total_weight);
        }
    }
}

// Helper function to find the path from root to a specific node
fn find_path_to_node<'a>(root: &'a TreeNode, target_path_parts: &[&'a str]) -> Option<Vec<&'a TreeNode>> {
    let mut current_node = root;
    let mut path_to_node = vec![root];

    for part in target_path_parts {
        if let Some(child) = current_node.children.get(*part) {
            current_node = child;
            path_to_node.push(child);
        } else {
            return None; // Path not found
        }
    }
    Some(path_to_node)
}

// Helper function to find the Lowest Common Ancestor (LCA) of two paths
fn find_lca<'a>(root: &'a TreeNode, path1_parts: &[&'a str], path2_parts: &[&'a str]) -> Option<&'a TreeNode> {
    let path1_nodes = find_path_to_node(root, path1_parts)?;
    let path2_nodes = find_path_to_node(root, path2_parts)?;

    let mut lca: Option<&TreeNode> = None;
    for (node1, node2) in path1_nodes.iter().zip(path2_nodes.iter()) {
        if node1.name == node2.name {
            lca = Some(node1);
        } else {
            break;
        }
    }
    lca
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Call ragit-feature-extractor to get sampled reports
    let sampled_reports = ragit_feature_extractor::get_sampled_reports()?;

    let mut root = TreeNode::new("root");

    for report in &sampled_reports {
        let path_parts: Vec<&str> = report.file_name.split('/').collect();
        root.add_path(&path_parts, report);
    }

    root.aggregate_term_counts();
    root.calculate_weights_and_fractions(0.0);

    // Print the tree
    println!("--- File Tree ---");
    println!("{:#?}", root);
    println!("-----------------");

    // Find and report common denominators (LCA) for sampled files
    println!("\n--- Common Denominators (LCA) for Sampled Files ---");
    for i in 0..sampled_reports.len() {
        for j in i + 1..sampled_reports.len() {
            let report1 = &sampled_reports[i];
            let report2 = &sampled_reports[j];

            let path1_parts: Vec<&str> = report1.file_name.split('/').collect();
            let path2_parts: Vec<&str> = report2.file_name.split('/').collect();

            if let Some(lca_node) = find_lca(&root, &path1_parts, &path2_parts) {
                println!("\nFiles: \"{}\" and \"{}\"", report1.file_name, report2.file_name);
                println!("  LCA: {}", lca_node.name);

                if let Some(path_to_lca1) = find_path_to_node(&root, &path1_parts) {
                    let lca_index = path_to_lca1.iter().position(|&n| n.name == lca_node.name).unwrap_or(0);
                    println!("  Path from LCA to {}: {}", report1.file_name, path_to_lca1[lca_index..].iter().map(|n| n.name.as_str()).collect::<Vec<&str>>().join("/"));
                }
                if let Some(path_to_lca2) = find_path_to_node(&root, &path2_parts) {
                    let lca_index = path_to_lca2.iter().position(|&n| n.name == lca_node.name).unwrap_or(0);
                    println!("  Path from LCA to {}: {}", report2.file_name, path_to_lca2[lca_index..].iter().map(|n| n.name.as_str()).collect::<Vec<&str>>().join("/"));
                }
            }
        }
    }
    println!("-----------------------------------------------------");

    Ok(())
}