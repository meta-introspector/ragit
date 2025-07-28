use anyhow::Result;
use std::path::PathBuf;
use glob;

pub trait FileSource {
    fn get_files(&self) -> Result<Vec<String>>;
}

pub struct StaticFileSource {
    pub files: Vec<String>,
}

impl FileSource for StaticFileSource {
    fn get_files(&self) -> Result<Vec<String>> {
        Ok(self.files.clone())
    }
}

pub struct CargoPackageFileSource {
    pub package_name: String,
    pub project_root: String,
}

impl FileSource for CargoPackageFileSource {
    fn get_files(&self) -> Result<Vec<String>> {
        let package_path = PathBuf::from(&self.project_root).join("crates").join("layer7_application").join("commands").join(&self.package_name);
        let pattern = format!("{}/**/*.rs", package_path.to_string_lossy());
        let mut files = Vec::new();
        for entry in glob::glob(&pattern)? {
            match entry {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        println!("CargoPackageFileSource: Found files: {:?}", files);
        Ok(files)
    }
}

pub struct GlobFileSource {
    pub pattern: String,
}

impl FileSource for GlobFileSource {
    fn get_files(&self) -> Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in glob::glob(&self.pattern)? {
            match entry {
                Ok(path) => {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(files)
    }
}
