use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use serde_json;
use path_clean::PathClean;
use num_enum::{IntoPrimitive, FromPrimitive};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
enum RelationshipType {
    #[num_enum(default)]
    NoRelationship = 0,
    Subdirectory = 1,
    SameFilename = 2,
    ContentSimilarity = 3, // Placeholder for future content analysis
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
enum UsageCount {
    #[num_enum(default)]
    Zero = 0,
    Few = 1,
    Many = 2,
    All = 3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TermAttributes {
    ignore: bool,
    key: bool,
    boring: bool,
    usage_count: UsageCount,
}

impl Default for TermAttributes {
    fn default() -> Self {
        TermAttributes {
            ignore: false,
            key: false,
            boring: false,
            usage_count: UsageCount::Zero,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PathMatrix {
    paths: Vec<String>,
    attributes: Vec<TermAttributes>,
    matrix: Vec<Vec<RelationshipType>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit");
    let mut all_paths: Vec<PathBuf> = Vec::new();
    let mut path_to_index: HashMap<PathBuf, usize> = HashMap::new();
    let mut index_counter = 0;

    let ignored_patterns = vec![
        ".git",
        "target",
        "node_modules",
        "__pycache__",
        ".DS_Store",
        ".idea",
        ".vscode",
        ".ragit",
        "vendor",
        "index/solfunmeme-index",
        "logs",
        "tmp_bootstrap",
        "comms",
        "RelNotes",
    ];

    println!("Collecting paths from: {}", root_dir.display());
    for entry in WalkDir::new(&root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_path_buf().clean();
        let relative_path = path.strip_prefix(&root_dir).unwrap_or(&path);
        let path_str = relative_path.to_string_lossy();

        let mut should_ignore = false;
        for pattern in &ignored_patterns {
            if path_str.contains(pattern) {
                should_ignore = true;
                break;
            }
        }

        if !should_ignore {
            all_paths.push(path.clone());
            path_to_index.insert(path, index_counter);
            index_counter += 1;
        }
    }

    let num_paths = all_paths.len();
    let mut matrix: Vec<Vec<RelationshipType>> = vec![vec![RelationshipType::NoRelationship; num_paths]; num_paths];
    let mut attributes: Vec<TermAttributes> = vec![TermAttributes::default(); num_paths];
    let mut relationship_counts: Vec<usize> = vec![0; num_paths];

    println!("Building relationship matrix ({}x{})...", num_paths, num_paths);
    for i in 0..num_paths {
        for j in 0..num_paths {
            if i == j {
                matrix[i][j] = RelationshipType::NoRelationship; // A path is not related to itself in this context
                continue;
            }

            let path_i = &all_paths[i];
            let path_j = &all_paths[j];

            // Relationship: Subdirectory
            if path_i.starts_with(path_j) && path_i != path_j {
                matrix[i][j] = RelationshipType::Subdirectory;
                relationship_counts[i] += 1;
                relationship_counts[j] += 1;
            } else if path_j.starts_with(path_i) && path_i != path_j {
                matrix[i][j] = RelationshipType::Subdirectory;
                relationship_counts[i] += 1;
                relationship_counts[j] += 1;
            }

            // Relationship: SameFilename
            if let (Some(name_i), Some(name_j)) = (path_i.file_name(), path_j.file_name()) {
                if name_i == name_j && path_i.parent() != path_j.parent() {
                    matrix[i][j] = RelationshipType::SameFilename;
                    relationship_counts[i] += 1;
                    relationship_counts[j] += 1;
                }
            }
        }
    }

    // Determine UsageCount
    let max_relationships = relationship_counts.iter().max().cloned().unwrap_or(0);
    for i in 0..num_paths {
        let count = relationship_counts[i];
        attributes[i].usage_count = match count {
            0 => UsageCount::Zero,
            _ if count as f64 / max_relationships as f64 > 0.75 => UsageCount::All,
            _ if count as f64 / max_relationships as f64 > 0.25 => UsageCount::Many,
            _ => UsageCount::Few,
        };
    }

    let path_strings: Vec<String> = all_paths.iter().map(|p| p.to_string_lossy().to_string()).collect();

    let result = PathMatrix {
        paths: path_strings,
        attributes,
        matrix,
    };

    let json_output = serde_json::to_string_pretty(&result)?;
    std::fs::write("path_relationship_matrix.json", json_output.as_bytes())?;
    println!("Matrix generated and saved to path_relationship_matrix.json");

    Ok(())
}
