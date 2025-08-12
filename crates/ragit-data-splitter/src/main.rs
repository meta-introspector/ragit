use ragit_data_splitter::{extract_and_save_elements, matrix_splitter};
use std::env;
use std::path::Path;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file_path> <output_directory>", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_dir = Path::new(&args[2]);

    if !input_path.exists() {
        eprintln!("Error: Input file does not exist: {}", input_path.display());
        std::process::exit(1);
    }

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    if input_path.file_name().map_or(false, |name| name == "path_relationship_matrix.json") {
        matrix_splitter::split_matrix_file(input_path, output_dir)?;
    } else {
        extract_and_save_elements(input_path, output_dir)?;
    }

    Ok(())
}
