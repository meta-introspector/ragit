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
        let project_root_path = PathBuf::from(&self.project_root);
        let patterns = [
            "src/**/*.rs",
            "crates/**/*.rs",
            "docs/**/*.md",
            "vendor/**/*.rs",
            "vendor/**/*.md",
        ];
        let mut files = Vec::new();
        for pattern_str in &patterns {
            let full_pattern = project_root_path.join(pattern_str).to_string_lossy().into_owned();
            for entry in glob::glob(&full_pattern)? {
                match entry {
                    Ok(path) => {
                        if let Some(path_str) = path.to_str() {
                            if let Ok(relative_path) = path.strip_prefix(&project_root_path) {
                                files.push(relative_path.to_string_lossy().into_owned());
                            }
                        }
                    }
                    Err(e) => return Err(e.into()),
                }
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