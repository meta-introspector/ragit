use anyhow::Result;
use serde::{Deserialize, Serialize};
use solfunmeme_core_logic::CodeVectorizer;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct TreeNode {
    r#type: String,
    name: Option<String>,
    contents: Option<Vec<TreeNode>>,
}

fn traverse_tree(node: &TreeNode, current_path: &Path, vectors: &mut HashMap<String, Vec<f32>>) {
    if node.r#type == "directory" {
        if let Some(name) = &node.name {
            let new_path = current_path.join(name);
            let vectorizer = CodeVectorizer::new(8);
            let vector = vectorizer.vectorize(new_path.to_str().unwrap_or(""));
            vectors.insert(new_path.to_string_lossy().to_string(), vector.dimensions);

            if let Some(contents) = &node.contents {
                for child in contents {
                    traverse_tree(child, &new_path, vectors);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let tree_file = File::open("tree_level_3.json")?;
    let tree_data: Vec<TreeNode> = serde_json::from_reader(tree_file)?;

    let mut directory_vectors = HashMap::new();
    for root_node in tree_data {
        traverse_tree(&root_node, Path::new(""), &mut directory_vectors);
    }

    let output_file_path = "directory_vectors.json";
    let mut output_file = File::create(output_file_path)?;
    let json_output = serde_json::to_string_pretty(&directory_vectors)?;
    output_file.write_all(json_output.as_bytes())?;

    println!("Directory vectors saved to {}", output_file_path);

    Ok(())
}