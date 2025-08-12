use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source directory to reorganize
    #[clap(short, long, value_parser)]
    source: String,

    /// Maximum number of items (files or directories) per subdirectory
    #[clap(short, long, value_parser, default_value_t = 100)]
    max_items: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_path = PathBuf::from(args.source);
    if !source_path.is_dir() {
        return Err(format!("Source path \'{}\' is not a directory.", source_path.display()).into());
    }

    println!("Reorganizing directory: {}", source_path.display());
    println!("Max items per subdirectory: {}", args.max_items);

    let mut current_subdir_count = 0;
    let mut current_subdir_path = PathBuf::new();
    let mut subdir_index = 0;

    for entry in WalkDir::new(&source_path).min_depth(1).max_depth(1) {
        let entry = entry?;
        let entry_path = entry.path();

        if current_subdir_count == 0 || current_subdir_count >= args.max_items {
            subdir_index += 1;
            current_subdir_path = source_path.join(format!("part_{:04}", subdir_index));
            fs::create_dir_all(&current_subdir_path)?;
            current_subdir_count = 0;
        }

        let destination_path = current_subdir_path.join(entry_path.file_name().unwrap());
        println!("  Moving {} to {}", entry_path.display(), destination_path.display());
        fs::rename(&entry_path, &destination_path)?;
        current_subdir_count += 1;
    }

    println!("Reorganization complete.");

    Ok(())
}
