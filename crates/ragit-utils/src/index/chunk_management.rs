use crate::chunk::{self, Chunk, ChunkBuildInfo, RenderedChunk, merge_and_convert_chunks};
use crate::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME, IMAGE_DIR_NAME, INDEX_DIR_NAME};
use crate::error::Error;
use crate::index::{tfidf, ImageDescription};
use crate::uid::{self, Uid};
use ragit_fs::{exists, extension, is_dir, join3, read_dir, read_bytes, read_string};
use serde_json::Value;
use std::collections::HashSet;

pub type Path = String;

impl super::Index {
    pub(crate) async fn load_chunks_or_tfidf(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Chunk>, Error> {
        if self.chunk_count > limit {
            let keywords = self.extract_keywords(query).await?;
            let tfidf_results = self.run_tfidf(
                keywords,
                limit,
            )?;
            let mut chunks = Vec::with_capacity(tfidf_results.len());

            for tfidf_result in tfidf_results.into_iter() {
                let uid = tfidf_result.id;
                chunks.push(self.get_chunk_by_uid(uid)?);
            }

            Ok(chunks)
        }

        else {
            let mut chunks = vec![];

            for chunk_path in self.get_all_chunk_files()? {
                chunks.push(chunk::load_from_file(&chunk_path)?);
            }

            Ok(chunks)
        }
    }

    pub fn get_all_chunk_files(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &CHUNK_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for chunk_file in read_dir(&internal, false)? {
                if extension(&chunk_file).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                    result.push(chunk_file.to_string());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_tfidf_files(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &CHUNK_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for tfidf_file in read_dir(&internal, false)? {
                if extension(&tfidf_file).unwrap_or(None).unwrap_or(String::new()) == "tfidf" {
                    result.push(tfidf_file.to_string());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_image_files(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &IMAGE_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for image_file in read_dir(&internal, false)? {
                if extension(&image_file).unwrap_or(None).unwrap_or(String::new()) == "png" {
                    result.push(image_file.to_string());
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub(crate) fn get_all_file_indexes(&self) -> Result<Vec<Path>, Error> {
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, &INDEX_DIR_NAME, &FILE_INDEX_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for file_index in read_dir(&internal, false)? {
                result.push(file_index.to_string());
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_chunk_by_uid(&self, uid: Uid) -> Result<Chunk, Error> {
        let chunk_at = self.get_uid_path(
            &self.root_dir,
            CHUNK_DIR_NAME,
            uid,
            Some("chunk"),
        )?;

        if exists(&chunk_at) {
            return Ok(chunk::load_from_file(&chunk_at)?);
        }

        Err(Error::NoSuchChunk(uid))
    }

    pub fn check_chunk_by_uid(&self, uid: Uid) -> bool {
        if let Ok(chunk_at) = self.get_uid_path(
            &self.root_dir,
            CHUNK_DIR_NAME,
            uid,
            Some("chunk"),
        ) {
            exists(&chunk_at)
        }

        else {
            false
        }
    }

    pub fn get_tfidf_by_chunk_uid(
        &self,
        uid: Uid,
    ) -> Result<ProcessedDoc, Error> {
        let tfidf_at = self.get_uid_path(
            &self.root_dir,
            CHUNK_DIR_NAME,
            uid,
            Some("tfidf"),
        )?;

        if exists(&tfidf_at) {
            return Ok(tfidf::load_from_file(&tfidf_at)?);
        }

        Err(Error::NoSuchChunk(uid))
    }

    pub fn get_tfidf_by_file_uid(
        &self,
        uid: Uid,
    ) -> Result<ProcessedDoc, Error> {
        let chunk_uids = self.get_chunks_of_file(uid)?;
        let mut result = ProcessedDoc::empty();

        for uid in chunk_uids.iter() {
            result.extend(&self.get_tfidf_by_chunk_uid(*uid)?);
        }

        result.uid = Some(uid);
        Ok(result)
    }

    pub fn get_chunks_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, Error> {
        let file_index_path = self.get_uid_path(
            &self.root_dir,
            FILE_INDEX_DIR_NAME,
            file_uid,
            None,
        )?;

        if exists(&file_index_path) {
            return Ok(uid::load_from_file(&file_index_path)?);
        }

        Err(Error::NoSuchFile { path: None, uid: Some(file_uid) })
    }

    pub fn get_merged_chunk_of_file(&self, file_uid: Uid) -> Result<RenderedChunk, Error> {
        let chunk_uids = self.get_chunks_of_file(file_uid)?;
        let mut chunks = Vec::with_capacity(chunk_uids.len());

        for chunk_uid in chunk_uids.iter() {
            chunks.push(self.get_chunk_by_uid(*chunk_uid)?);
        }

        // FIXME: I don't think we have to sort this
        chunks.sort_by_by_key(|chunk| chunk.sortable_string());
        let chunks = merge_and_convert_chunks(self, chunks)?;

        match chunks.len() {
            0 => todo!(),  // It's an empty file. Does ragit create a chunk for an empty file? I don't remember...
            1 => Ok(chunks[0].clone()),
            _ => Err(Error::BrokenIndex(format!("Internal error: `get_merged_chunk_of_file({file_uid})` returned multiple chunks"))),
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

    pub fn get_image_bytes_by_uid(&self, uid: Uid) -> Result<Vec<u8>, Error> {
        Ok(read_bytes(&self.get_uid_path(IMAGE_DIR_NAME, uid, Some("png"))?)?)
    }

    pub fn get_image_description_by_uid(&self, uid: Uid) -> Result<ImageDescription, Error> {
        let j = read_string(&self.get_uid_path(IMAGE_DIR_NAME, uid, Some("json"))?)?;
        let v = serde_json::from_str::<ImageDescription>(&j)?;
        Ok(v)
    }
}
