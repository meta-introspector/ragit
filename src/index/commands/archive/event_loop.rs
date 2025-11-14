use std::sync::mpsc;

use ragit_fs::{join3, WriteMode, write_bytes};
use serde_json::Value;

use crate::{chunk::Chunk, constant::{CHUNK_DIR_NAME, INDEX_DIR_NAME, INDEX_FILE_NAME}, index::index_struct::Index};

use super::{decompress::decompress, request::Request, response::Response, BlockType};

pub fn event_loop(
    tx_to_main: mpsc::Sender<Response>,
    rx_from_main: mpsc::Receiver<Request>,
    root_dir: String,
) -> Result<(), crate::error::Error> {
    for msg in rx_from_main {
        match msg {
            Request::Extract { block_type, path, from, to } => {
                let mut bytes = ragit_fs::read_bytes_offset(&path, from, to)?;
                bytes = decompress(&bytes)?;

                match block_type {
                    BlockType::Index => {
                        let index = serde_json::from_slice::<Value>(&bytes)?;
                        let index = serde_json::to_vec_pretty(&index)?;

                        write_bytes(
                            &join3(
                                &root_dir,
                                INDEX_DIR_NAME,
                                INDEX_FILE_NAME,
                            )?,
                            &index,
                            WriteMode::CreateOrTruncate,
                        )?;
                    },
                    BlockType::Chunk => {
                        let chunks = serde_json::from_slice::<Vec<Chunk>>(&bytes)?;

                        for chunk in chunks.iter() {
                            let chunk_at = Index::get_uid_path(
                                &root_dir,
                                CHUNK_DIR_NAME,
                                chunk.uid,
                                Some("chunk"),
                            )?;
                            let parent = ragit_fs::parent(&chunk_at)?;
                            ragit_fs::try_create_dir(&parent)?;
                            let chunk = serde_json::to_vec_pretty(chunk)?;
                            write_bytes(chunk_at.to_str().unwrap(), &chunk, WriteMode::CreateOrTruncate)?;
                        }
                    }
                    _ => todo!(),
                }
            }
            Request::TellMeWhenYouAreDone => {
                tx_to_main.send(Response::IAmDone)?;
            }
            Request::Kill => {
                break;
            }
        }
    }
    Ok(())
}