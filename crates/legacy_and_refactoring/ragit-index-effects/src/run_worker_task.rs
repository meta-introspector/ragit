use std::path::PathBuf;
use tokio::sync::mpsc;
use sha3::{Digest, Sha3_256};
use crate::build_chunks::build_chunks;
use crate::channel::{WorkerRequest, WorkerResponse};
use ragit_types::ApiError;

pub async fn run_worker_task(
    root_dir: PathBuf,
    tx_to_main: mpsc::UnboundedSender<WorkerResponse>,
    mut rx_from_main: mpsc::UnboundedReceiver<WorkerRequest>,
) -> Result<(), anyhow::Error> {
    let mut index = match ragit_index_types::index_struct::Index::load(
        root_dir.clone(),
        ragit_index_types::load_mode::LoadMode::OnlyJson,
    ) {
        Ok(index) => index,
        Err(e) => {
            let api_err: ApiError = e.into();
            let _ = tx_to_main.send(WorkerResponse::Error(api_err.clone()));
            drop(tx_to_main);
            return Err(api_err.into());
        },
    };
    if let Err(e) = ragit_index_types::index_impl::load_prompts::load_prompts_from_directory(&mut index, &root_dir.join("prompts")) {
        let api_err = ApiError::from(e);
        let _ = tx_to_main.send(WorkerResponse::Error(api_err.clone()));
        drop(tx_to_main);
        return Err(anyhow::anyhow!("Failed to load prompts: {}", api_err.clone()));
    }
    let prompt = index.prompts.get("summarize").ok_or_else(|| anyhow::anyhow!("Prompt 'summarize' not found"))?.clone();
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
    Ok(())
}
