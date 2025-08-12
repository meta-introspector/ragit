use anyhow::Result;
use std::path::PathBuf;
use glob;

pub trait FileSource {
    fn get_files(&self, memory_monitor: &mut ragit_memory_monitor::MemoryMonitor) -> Result<Vec<String>>;
}

pub struct StaticFileSource {
    pub files: Vec<String>,
}

impl FileSource for StaticFileSource {
    fn get_files(&self, memory_monitor: &mut ragit_memory_monitor::MemoryMonitor) -> Result<Vec<String>> {
        memory_monitor.verbose("StaticFileSource: Getting files from static source.");
        Ok(self.files.clone())
    }
}

pub struct CargoPackageFileSource {
    pub package_name: String,
    pub project_root: String,
}

impl FileSource for CargoPackageFileSource {
    fn get_files(&self, memory_monitor: &mut ragit_memory_monitor::MemoryMonitor) -> Result<Vec<String>> {
        memory_monitor.verbose(&format!("CargoPackageFileSource: Getting files for package: {}", self.package_name));
        let package_path = PathBuf::from(&self.project_root).join("crates").join("layer7_application").join("commands").join(&self.package_name);
        memory_monitor.verbose(&format!("CargoPackageFileSource: Package path: {:?}", package_path));
        let pattern = format!("{}/**/*.rs", package_path.to_string_lossy());
        memory_monitor.verbose(&format!("CargoPackageFileSource: Glob pattern: {}", pattern));
        let mut files = Vec::new();
        for entry in glob::glob(&pattern)? {
            match entry {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                        memory_monitor.verbose(&format!("CargoPackageFileSource: Found file: {}", path_str));
                    }
                }
                Err(e) => {
                    memory_monitor.verbose(&format!("CargoPackageFileSource: Error reading glob entry: {}", e));
                    return Err(e.into())
                },
            }
        }
        memory_monitor.verbose(&format!("CargoPackageFileSource: Found {} files.", files.len()));
        Ok(files)
    }
}

pub struct GlobFileSource {
    pub pattern: String,
}

impl FileSource for GlobFileSource {
    fn get_files(&self, memory_monitor: &mut ragit_memory_monitor::MemoryMonitor) -> Result<Vec<String>> {
        memory_monitor.verbose(&format!("GlobFileSource: Getting files for pattern: {}", self.pattern));
        let mut files = Vec::new();
        for entry in glob::glob(&self.pattern)? {
            match entry {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                        memory_monitor.verbose(&format!("GlobFileSource: Found file: {}", path_str));
                    }
                }
                Err(e) => {
                    memory_monitor.verbose(&format!("GlobFileSource: Error reading glob entry: {}", e));
                    return Err(e.into())
                },
            }
        }
        memory_monitor.verbose(&format!("GlobFileSource: Found {} files.", files.len()));
        Ok(files)
    }
}

