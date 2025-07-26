use anyhow::Result;
use std::process::Command;

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
}

impl FileSource for CargoPackageFileSource {
    fn get_files(&self) -> Result<Vec<String>> {
        let output = Command::new("cargo")
            .arg("pkgid")
            .arg("--verbose")
            .arg(&self.package_name)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to get package files for {}: {}",
                self.package_name,
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let output = String::from_utf8(output.stdout)?;
        let files = output
            .lines()
            .filter(|line| line.starts_with("file://"))
            .map(|line| line.trim_start_matches("file://").to_string())
            .collect();

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
