use ragit_index_types::index_struct::Index;

//use ragit_types::ApiError;
use std::path::PathBuf;
use tokio::sync::mpsc;
use sha3::{Digest, Sha3_256};
use crate::build_chunks::build_chunks;

use crate::channel::{Channel, WorkerRequest, WorkerResponse};
use ragit_index_types::index_get_prompt;

pub fn init_worker(root_dir: PathBuf) -> Channel {
    let (tx_to_main, rx_to_main) = mpsc::unbounded_channel();
    let (tx_from_main, mut rx_from_main) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        // Each process requires an instance of `Index`, but I found
        // it too difficult to send the instance via mpsc channels.
        // So I'm just instantiating new ones here.
        // Be careful not to modify the index!
        // Temporarily creating a new Index instead of loading from disk to debug memory usage.
        // let mut index = match Index::load(
        //     root_dir,
        //     LoadMode::OnlyJson,
        // ) {
        //     Ok(index) => index,
        //     Err(e) => {
        //         let _ = tx_to_main.send(WorkerResponse::Error(e));
        //         drop(tx_to_main);
        //         return;
        //     },
        // };
        let mut index = Index::new(root_dir.clone());
        if let Err(e) = ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(&mut index, &root_dir.join("prompts")) {
            let _ = tx_to_main.send(WorkerResponse::Error(e.into()));
            drop(tx_to_main);
            return;
        }
        let prompt = match index_get_prompt(&index,"summarize") {
            Ok(prompt) => prompt,
            Err(e) => {
                let _ = tx_to_main.send(WorkerResponse::Error(e.into()));
                drop(tx_to_main);
                return;
            },
        };
        let mut hasher = Sha3_256::new();
        hasher.update(prompt.as_bytes());
        let prompt_hash = format!("{:064x}", hasher.finalize());

        while let Some(msg) = rx_from_main.recv().await {
            match msg {
                WorkerRequest::BuildChunks { file, dry_run_llm } => match build_chunks(
                    &mut index,
                    file,
                    prompt_hash.clone(),
                    tx_to_main.clone(),
                    dry_run_llm,
                ).await {
                    Ok(_) => {},
                    Err(e) => {
                        if tx_to_main.send(WorkerResponse::Error(e.into())).is_err() {
                            // the parent process is dead
                            break;
                        }
                    },
                },
                WorkerRequest::Kill => { break; },
            }
        }

        drop(tx_to_main);
        return;
    });

    Channel {
        rx_to_main,
        tx_from_main,
    }
}
