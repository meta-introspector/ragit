use anyhow::Result;
use std::path::PathBuf;
use ragit_index_types::index_struct::Index;
use ragit_types::build_config::BuildConfig;
use ragit_readers::{PlainTextReader, FileReaderImpl};

use crate::bootstrap_commands::process_staged_file_logic::create_chunk_from_file_data::create_chunk_from_file_data;

pub fn process_staged_file(
    verbose: bool,
    file_path_buf: &PathBuf,
    _actual_root_dir: &PathBuf,
    build_config: &BuildConfig,
    index: &mut Index,
) -> Result<(), anyhow::Error> {
    let file_path = file_path_buf.to_string_lossy().to_string();
    if verbose {
        println!("bootstrap_index_self: Processing staged file: {:?}", file_path);
    }

    let mut reader: PlainTextReader = PlainTextReader::new(file_path_buf.to_str().unwrap(), &index.root_dir.to_string_lossy(), build_config)?;
    reader.load_tokens()?;
    let tokens = reader.pop_all_tokens()?;
    let file_content = tokens.into_iter().filter_map(|token| {
        if let ragit_types::chunk::atomic_token::AtomicToken::String { data, .. } = token {
            Some(data)
        } else {
            None
        }
    }).collect::<Vec<String>>().join("");

    let chunk = create_chunk_from_file_data(
        file_content,
        file_path,
        index.chunk_count,
        file_path_buf,
    )?;
    index.add_chunk(chunk);

    Ok(())
}