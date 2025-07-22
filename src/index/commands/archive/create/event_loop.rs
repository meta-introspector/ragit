use crate::prelude::*;
use crate::index::commands::archive::create::request::Request;
use crate::index::commands::archive::create::response::Response;
use crate::index::commands::archive::BlockType;
use std::sync::mpsc;
use ragit_fs::{read_string, join3, write_bytes, WriteMode};
use serde_json::Value;
use crate::index::IIStatus;

fn event_loop(
    tx_to_main: mpsc::Sender<Response>,
    rx_from_main: mpsc::Receiver<Request>,
    root_dir: String,
    worker_id: usize,
    compression_level: u32,
) -> Result<(), Error> {
    let index = Index::load(root_dir, LoadMode::OnlyJson)?;
    let mut seq = 0;

    for msg in rx_from_main {
        match msg {
            Request::Compress(block_type, uids) => {
                let block_data = match block_type {
                    BlockType::Index => {
                        let index_json = read_string(&join3(
                            index.root_dir.to_str().unwrap(),
                            INDEX_DIR_NAME,
                            INDEX_FILE_NAME,
                        )?)?;
                        let mut index = serde_json::from_str::<Index>(&index_json)?;

                        index.ii_status = IIStatus::None;
                        index.uid = Some(index.calculate_uid(false)?);

                        let index_json = serde_json::to_vec(&index)?;
                        decompress(&index_json)?
                    },
                    BlockType::Chunk => {
                        let mut chunks = Vec::with_capacity(uids.len());
                        for uid in uids.iter() {
                            chunks.push(index.get_chunk_by_uid(*uid)?);
                        }
                        let bytes = serde_json::to_vec(&chunks)?;
                        decompress(&bytes)?
                    },
                    BlockType::ImageBytes => {
                        let mut images = HashMap::with_capacity(uids.len());
                        for uid in uids.iter() {
                            images.insert(
                                uid.to_string(),
                                encode_base64(&index.get_image_bytes_by_uid(*uid)?),
                            );
                        }
                        let bytes = serde_json::to_vec(&images)?;
                        decompress(&bytes)?
                    },
                    BlockType::ImageDesc => {
                        let mut descs = HashMap::with_capacity(uids.len());
                        for uid in uids.iter() {
                            descs.insert(uid.to_string(), index.get_image_description_by_uid(*uid)?);
                        }
                        let bytes = serde_json::to_vec(&descs)?;
                        decompress(&bytes)?
                    },
                    BlockType::Meta => {
                        let meta = index.get_all_meta()?;
                        if meta.is_empty() {
                            vec![]
                        } else {
                            let bytes = serde_json::to_vec(&meta)?;
                            decompress(&bytes)?
                        }
                    },
                    BlockType::Prompt => {
                        let bytes = serde_json::to_vec(&index.prompts)?;
                        decompress(&bytes)?
                    },
                    BlockType::Config => {
                        let mut obj = Map::new();
                        obj.insert(String::from("api"), serde_json::to_value(&index.api_config)?);
                        obj.insert(String::from("build"), serde_json::to_value(&index.build_config)?);
                        obj.insert(String::from("query"), serde_json::to_value(&index.query_config)?);
                        let bytes = serde_json::to_vec(&obj)?;
                        decompress(&bytes)?
                    },
                    BlockType::Splitted { .. } => unreachable!(),
                    BlockType::Tfidf => todo!(),
                    BlockType::Metadata => todo!(),
                };

                if !block_data.is_empty() {
                    let block_file_name = format!("__archive_block_{worker_id:06}_{seq:06}");
                    write_bytes(
                        &block_file_name,
                        &block_data,
                        WriteMode::AlwaysCreate,
                    )?;
                    seq += 1;

                    tx_to_main.send(Response::Compressed(block_type, block_file_name)).map_err(|_| Error::MPSCError(String::from("Failed to send response to main")))?;
                }
            },
            Request::TellMeWhenYouAreDone => {
                tx_to_main.send(Response::IAmDone).map_err(|_| Error::MPSCError(String::from("Failed to send response to main")))?;
            },
            Request::Kill => { break; },
        }
    }

    drop(tx_to_main);
    Ok(())
}
