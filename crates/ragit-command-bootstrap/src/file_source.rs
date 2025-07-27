use anyhow::Result;
use std::process::Command;
use std::path::Path;

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
        println!("CargoPackageFileSource: Getting files for package '{}' in project root '{}'", self.package_name, self.project_root);
        let output = Command::new("cargo")
            .current_dir(&self.project_root)
            .arg("pkgid")
            .arg(&self.package_name)
            .output()?;

        if !output.status.success() {
            println!("CargoPackageFileSource: Failed to get package files for {}: {}", self.package_name, String::from_utf8_lossy(&output.stderr));
            return Err(anyhow::anyhow!(
                "Failed to get package files for {}: {}",
                self.package_name,
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let output_str = String::from_utf8(output.stdout)?;
        println!("CargoPackageFileSource: 'cargo pkgid' output:\n{}", output_str);

        let pkgid = output_str.trim();

        let metadata_output = Command::new("cargo")
            .current_dir(&self.project_root)
            .arg("metadata")
            .arg("--format-version=1")
            .output()?;

        if !metadata_output.status.success() {
            println!("CargoPackageFileSource: Failed to get metadata: {}", String::from_utf8_lossy(&metadata_output.stderr));
            return Err(anyhow::anyhow!(
                "Failed to get metadata: {}",
                String::from_utf8_lossy(&metadata_output.stderr)
            ));
        }

        let metadata_str = String::from_utf8(metadata_output.stdout)?;
        let metadata: serde_json::Value = serde_json::from_str(&metadata_str)?;

        let mut files = Vec::new();
        if let Some(packages) = metadata["packages"].as_array() {
            for package in packages {
                if package["id"].as_str() == Some(pkgid) {
                    if let Some(targets) = package["targets"].as_array() {
                        for target in targets {
                            if let Some(src_path) = target["src_path"].as_str() {
                                files.push(src_path.to_string());
                            }
                        }
                    }
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

