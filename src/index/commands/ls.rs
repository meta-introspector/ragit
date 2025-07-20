use crate::index::index_struct::Index;
use crate::error::Error;
use crate::schema::{ChunkSchema, ImageSchema};
use crate::prelude::*;
use ragit_fs::{file_name, parent};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum LsResult {
    Chunk(ChunkSchema),
    Image(ImageSchema),
}

pub fn ls(
    index: &Index,
    all: bool,
    json: bool,
) -> Result<(), Error> {
    let mut result = vec![];

    for chunk_file in &index.get_all_chunk_files()? {
        let chunk = crate::chunk::load_from_file(&chunk_file)?;
        let chunk: ChunkSchema = chunk.into();
        result.push(LsResult::Chunk(chunk));
    }

    for image in &index.get_all_image_files()? {
        let image = index.get_image_schema(
            Uid::from_prefix_and_suffix(
                &file_name(&parent(&image)?)?,
                &file_name(image.to_str().unwrap())?,
            )?,
            false,  // load bytes
        )?;
        result.push(LsResult::Image(image));
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    else {
        for item in result.iter() {
            match item {
                LsResult::Chunk(chunk) => {
                    println!("chunk: {}", chunk.uid);
                },
                LsResult::Image(image) => {
                    println!("image: {}", image.uid);
                },
            }
        }
    }

    Ok(())
}
