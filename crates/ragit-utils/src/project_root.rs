use std::path::PathBuf;
use crate::error::Error;
use ragit_fs::{exists, parent, current_dir};
use crate::constant::INDEX_DIR_NAME;
use std::env;

pub fn find_root() -> Result<PathBuf, Error> {
    if let Ok(root_env) = env::var("RAGIT_ROOT") {
        let env_path = PathBuf::from(root_env);
        if exists(&env_path) {
            return Ok(env_path);
        } else {
            return Err(Error::CliError(crate::cli_types::CliError::new_message(
                format!("RAGIT_ROOT environment variable points to a non-existent directory: {}", env_path.display()),
            )));
        }
    }

    let current_path_str = current_dir()?;
    let mut current_path = PathBuf::from(current_path_str);

    loop {
        let ragit_dir = current_path.join(INDEX_DIR_NAME);
        if exists(&ragit_dir) {
            return Ok(current_path);
        }
        match parent(&current_path) {
            Ok(p) => current_path = p,
            Err(_) => {
                return Err(Error::CliError(crate::cli_types::CliError::new_message(
                    "Not a ragit repository (or any of the parent directories): .ragit not found!".to_string(),
                )));
            }
        }
    }
}
