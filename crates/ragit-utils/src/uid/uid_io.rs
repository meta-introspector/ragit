use std::path::PathBuf;

use crate::error::Error;
use crate::uid::Uid;
use crate::uid::UidWriteMode;
use ragit_fs::{read_bytes, write_bytes, WriteMode};

pub fn load_from_file(path: &PathBuf) -> Result<Vec<Uid>, Error> {
    let bytes = read_bytes(path.to_str().unwrap())?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn save_to_file(path: &PathBuf, uids: &Vec<Uid>, write_mode: UidWriteMode) -> Result<(), Error> {
    let bytes = serde_json::to_vec_pretty(uids)?;
    match write_mode {
        UidWriteMode::Naive => write_bytes(path.to_str().unwrap(), &bytes, WriteMode::AlwaysCreate)?,
        UidWriteMode::Compact => write_bytes(path.to_str().unwrap(), &bytes, WriteMode::AlwaysCreate)?,
    }
    Ok(())
}
