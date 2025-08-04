use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use ragit_core::grand_plan::rust_ast_mapping::ast_mapper::ast_mapper_struct::AstMapper;

fn main() {
    let existing_terms = get_existing_glossary_terms();
    let potential_terms = find_potential_glossary_terms();

    let missing_terms: Vec<_> = potential_terms
        .difference(&existing_terms)
        .collect();

    if missing_terms.is_empty() {
        println!("No missing glossary terms found.");
    } else {
        println!("Found {} missing glossary terms:", missing_terms.len());
        for term in missing_terms {
            println!("- {}", term);
        }
    }
}

fn get_existing_glossary_terms() -> HashSet<String> {
    let mut existing_terms = HashSet::new();
    let glossary_path = Path::new("docs/index/glossary_terms");
    if let Ok(entries) = fs::read_dir(glossary_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    existing_terms.insert(file_name.replace(".md", ""));
                }
            }
        }
    }
    existing_terms
}

fn find_potential_glossary_terms() -> HashSet<String> {
    let mut potential_terms = HashSet::new();
    let walker = WalkDir::new("crates")
        .into_iter()
        .chain(WalkDir::new("src").into_iter());

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(identifiers) = AstMapper::extract_public_identifiers(&content) {
                    for id in identifiers {
                        potential_terms.insert(id);
                    }
                } else {
                    eprintln!("Warning: Could not extract identifiers from {}. Skipping.", path.display());
                }
            }
        }
    }
    potential_terms
}
