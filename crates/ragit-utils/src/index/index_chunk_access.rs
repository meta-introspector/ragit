use crate::chunk;
use crate::chunk::chunk_struct::Chunk;
use crate::chunk::rendered_chunk::RenderedChunk;
use crate::chunk::utils::merge_and_convert_chunks;

use crate::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME};
use crate::error::Error;
use crate::path_utils::get_uid_path;
use ragit_fs::exists;
use ragit_uid::Uid;
use std::collections::HashSet;
//use std::path::{Path, PathBuf};
use crate::index::path_management::Path;
use crate::index::{index_struct::Index, tfidf};

impl Index {
    pub fn get_chunk_by_uid(&self, uid: Uid) -> Result<Chunk, Error> {
        let chunk_at = get_uid_path(
            &self.root_dir,
            Path::new(CHUNK_DIR_NAME),
            uid,
            Some("chunk"),
        )?;

        if exists(&chunk_at) {
            return Ok(chunk::load_from_file(&chunk_at)?);
        }

        Err(Error::NoSuchChunk(uid))
    }

    pub fn check_chunk_by_uid(&self, uid: Uid) -> bool {
        if let Ok(chunk_at) = get_uid_path(
            &self.root_dir,
            Path::new(CHUNK_DIR_NAME),
            uid,
            Some("chunk"),
        ) {
            exists(&chunk_at)
        } else {
            false
        }
    }

    pub fn get_tfidf_by_chunk_uid(&self, uid: Uid) -> Result<tfidf::ProcessedDoc, Error> {
        let tfidf_at = get_uid_path(
            &self.root_dir,
            Path::new(CHUNK_DIR_NAME),
            uid,
            Some("tfidf"),
        )?;

        if exists(&tfidf_at) {
            return Ok(tfidf::load_from_file(tfidf_at.to_str().unwrap())?);
        }

        Err(Error::NoSuchChunk(uid))
    }

    pub fn get_chunks_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, Error> {
        let file_index_path = get_uid_path(
            &self.root_dir,
            Path::new(FILE_INDEX_DIR_NAME),
            file_uid,
            None,
        )?;

        if exists(&file_index_path) {
            return Ok(ragit_uid::load_from_file(&file_index_path)?);
        }

        Err(Error::NoSuchFile {
            path: None,
            uid: Some(file_uid),
        })
    }

    pub fn get_merged_chunk_of_file(&self, file_uid: Uid) -> Result<RenderedChunk, Error> {
        let chunk_uids = self.get_chunks_of_file(file_uid)?;
        let mut chunks_raw = Vec::with_capacity(chunk_uids.len());

        for chunk_uid in chunk_uids.iter() {
            chunks_raw.push(self.get_chunk_by_uid(*chunk_uid)?);
        }

        // FIXME: I don't think we have to sort this
        chunks_raw.sort_by_key(|chunk| chunk.sortable_string());

        let mut chunks_rendered = Vec::with_capacity(chunks_raw.len());
        for chunk in chunks_raw {
            chunks_rendered.push(chunk.render(self)?);
        }

        let chunks = merge_and_convert_chunks(self, chunks_rendered)?;

        match chunks.len() {
            0 => todo!(), // It's an empty file. Does ragit create a chunk for an empty file? I don't remember...
            1 => Ok(chunks[0].clone()),
            _ => Err(Error::BrokenIndex(format!(
                "Internal error: `get_merged_chunk_of_file({file_uid})` returned multiple chunks"
            ))),
        }
    }

    pub fn get_images_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, Error> {
        let chunk_uids = self.get_chunks_of_file(file_uid)?;
        let mut result = HashSet::new();

        for chunk_uid in chunk_uids.into_iter() {
            let chunk = self.get_chunk_by_uid(chunk_uid)?;

            for image in chunk.images.iter() {
                result.insert(*image);
            }
        }

        Ok(result.into_iter().collect())
    }
}
