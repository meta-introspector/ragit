use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};

//use ragit_index_types::index_struct::Index;
use super::file_source::{FileSource, CargoPackageFileSource, GlobFileSource};

use super::constants::BOOTSTRAP_PACKAGE_NAME;
use ragit_memory_monitor::MemoryMonitor;

pub async fn add_bootstrap_files(
    actual_root_dir: &PathBuf,
    max_memory_gb: Option<u64>,
    memory_monitor: Arc<Mutex<MemoryMonitor>>,
    max_files_to_process: Option<usize>,
    target: Option<String>,
) -> Result<mpsc::Receiver<PathBuf>, anyhow::Error> {
    memory_monitor.lock().unwrap().verbose("bootstrap_index_self: Running rag add");
    memory_monitor.lock().unwrap().verbose(&format!("bootstrap_index_self: Found project root: {:?}", actual_root_dir));

    let (tx, rx) = mpsc::channel(100); // Create a channel with a buffer of 100

    let files_to_add_source: Box<dyn FileSource> = match target.as_deref() {
        Some("all") => {
            memory_monitor.lock().unwrap().verbose("Using GlobFileSource for all Rust, Markdown, and TOML files.");
            Box::new(GlobFileSource {
                patterns: vec!["**/*.rs".to_string(), "**/*.md".to_string(), "**/*.toml".to_string()],
            })
        },
        Some("cargo-toml") => {
            memory_monitor.lock().unwrap().verbose("Using GlobFileSource for Cargo.toml files.");
            Box::new(GlobFileSource {
                patterns: vec!["**/Cargo.toml".to_string()],
            })
        },
        _ => {
            memory_monitor.lock().unwrap().verbose("Using CargoPackageFileSource.");
            Box::new(CargoPackageFileSource {
                package_name: BOOTSTRAP_PACKAGE_NAME.to_string(),
                project_root: actual_root_dir.to_str().unwrap().to_string(),
            })
        }
    };

    let mut files_to_add = files_to_add_source.get_files(&mut memory_monitor.lock().unwrap())?;

    if let Some(max_files) = max_files_to_process {
        if files_to_add.len() > max_files {
            files_to_add.truncate(max_files);
        }
    }
    memory_monitor.lock().unwrap().verbose(&format!("CargoPackageFileSource: Found {} files.", files_to_add.len()));

    let actual_root_dir_clone = actual_root_dir.clone();
    let memory_monitor_clone = memory_monitor.clone();

    tokio::spawn(async move {
        for p in files_to_add {
            let abs_path = actual_root_dir_clone.join(&p);
            if abs_path.is_file() {
                if let Err(e) = tx.send(abs_path).await {
                    eprintln!("Failed to send file path through channel: {}", e);
                    break;
                }
            }
        }
    });

    Ok(rx)
}
