use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let glossary_dir = Path::new("docs/index/glossary_terms");

    let header_template = "- **Emoji:** ❓\n- **Vector Locations:**\n    - **8D:** `[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]`\n    - **23D:** `[0.0, ...]`\n    - **41D:** `[0.0, ...]`\n    - **800D:** `[0.0, ...]`\n\n---\n\n";

    for entry in fs::read_dir(glossary_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let original_content = fs::read_to_string(&path)?;
            let new_content = format!("{}{}", header_template, original_content);
            fs::write(&path, new_content)?;
            println!("Updated {:?}", path.file_name().unwrap());
        }
    }

    println!("\nGlossary update complete.");
    Ok(())
}
