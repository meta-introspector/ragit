use crate::error::Error;
use crate::index::index_struct::Index;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum SummaryMode {
    All,
    File,
}

pub type Summary = String; // Define Summary as an alias for String

pub async fn summary(
    index: &mut Index,
    mode: Option<SummaryMode>,
) -> Result<(), Error> {
    let started_at = Instant::now();

    index.summary(mode).await?;

    let elapsed_time = Instant::now().duration_since(started_at).as_secs();
    println!("---");
    println!("elapsed time: {:02}:{:02}", elapsed_time / 60, elapsed_time % 60);

    Ok(())
}


