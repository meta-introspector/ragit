use crate::get_build_options;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub compatible: bool,
}

pub fn get_compatibility_warning(version_info: &VersionInfo) -> Option<String> {
    // Placeholder implementation
    if !version_info.compatible {
        Some(format!("Warning: Incompatible version detected. Current version: {}, Expected compatible version.", version_info.version))
    } else {
        None
    }
}

pub fn version() {
    let build_options = get_build_options();
    println!("ragit version: {}", build_options.version);
    println!("profile: {}", build_options.profile);

    for (feature, enabled) in build_options.features.iter() {
        println!("feature {}: {}", feature, enabled);
    }
}
