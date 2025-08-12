use candle_core::Error;
use candle_datasets::nlp::hf_hub::{Repo, RepoType};

fn main() -> Result<(), Error> {
    let repo = Repo::new("idx".to_string(), RepoType::Dataset, "main".to_string());
    let dataset = repo.get("augmented_terms.jsonl")?;
    println!("Dataset: {:?}", dataset);
    Ok(())
}