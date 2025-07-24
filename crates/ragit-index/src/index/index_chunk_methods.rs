use crate::prelude::*;
// use ragit_utils::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME, IMAGE_DIR_NAME, INDEX_DIR_NAME};
// use ragit_utils::ragit_path_utils::{join3, get_uid_path};
// use ragit_fs::{exists, read_dir, is_dir, extension, read_bytes, read_string};
// use ragit_types::chunk::chunk_struct::Chunk;
// use ragit_types::chunk::rendered_chunk::RenderedChunk;
// //use ragit_types::chunk::merge_and_convert_chunks;
// use ragit_types::uid::Uid;
// use ragit_types::api_error::ApiError;
// use ragit_types::image::ImageSchema;
// use ragit_uid::load_from_file;
// use std::path::PathBuf;
// use std::collections::HashSet;
//use ragit_types::tfidf::ProcessedDoc;

impl super::Index {
    pub(crate) async fn load_chunks_or_tfidf(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Chunk>, ApiError> {
        // Assuming `chunk_count` is now a method or accessed differently
        // For now, let's assume it's a method `self.get_chunk_count()`
        // You'll need to implement `get_chunk_count` in `Index` if it's not there.
        // if self.get_chunk_count() > limit {
        //     let keywords = self.extract_keywords(query).await?;
        //     let tfidf_results = self.run_tfidf(
        //         keywords,
        //         limit,
        //     )?;
        //     let mut chunks = Vec::with_capacity(tfidf_results.len());

        //     for tfidf_result in tfidf_results.into_iter() {
        //         let uid = tfidf_result.doc_id; // Assuming `id` changed to `doc_id`
        //         chunks.push(self.get_chunk_by_uid(uid)?);
        //     }

        //     Ok(chunks)
        // }

        // else {
            let mut chunks = vec![];

            for chunk_path in self.get_all_chunk_files()?.into_iter() { // Added .into_iter()
                chunks.push(load_from_file(&chunk_path.to_string_lossy().into_owned())?); // Convert PathBuf to String
            }

            Ok(chunks)
        // }
    }

    pub fn get_all_chunk_files(&self) -> Result<Vec<PathBuf>, ApiError> { // Changed Path to PathBuf
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, INDEX_DIR_NAME, CHUNK_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for chunk_file in read_dir(&internal, false)? {
                if extension(&chunk_file).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                    result.push(chunk_file); // Push PathBuf directly
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_tfidf_files(&self) -> Result<Vec<PathBuf>, ApiError> { // Changed Path to PathBuf
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, INDEX_DIR_NAME, CHUNK_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for tfidf_file in read_dir(&internal, false)? {
                if extension(&tfidf_file).unwrap_or(None).unwrap_or(String::new()) == "tfidf" {
                    result.push(tfidf_file); // Push PathBuf directly
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_all_image_files(&self) -> Result<Vec<PathBuf>, ApiError> { // Changed Path to PathBuf
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, INDEX_DIR_NAME, IMAGE_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for image_file in read_dir(&internal, false)? {
                if extension(&image_file).unwrap_or(None).unwrap_or(String::new()) == "png" {
                    result.push(image_file); // Push PathBuf directly
                }
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub(crate) fn get_all_file_indexes(&self) -> Result<Vec<PathBuf>, ApiError> { // Changed Path to PathBuf
        let mut result = vec![];

        for internal in read_dir(&join3(&self.root_dir, INDEX_DIR_NAME, FILE_INDEX_DIR_NAME)?, false)? {
            if !is_dir(&internal) {
                continue;
            }

            for file_index in read_dir(&internal, false)? {
                result.push(file_index); // Push PathBuf directly
            }
        }

        // the result has to be deterministic
        result.sort();
        Ok(result)
    }

    pub fn get_chunk_by_uid(&self, uid: Uid) -> Result<Chunk, ApiError> {
        let chunk_at = self.get_uid_path(
            &self.root_dir,
            CHUNK_DIR_NAME,
            uid,
            Some("chunk"),
        )?;

        if exists(&chunk_at) {
            return Ok(load_from_file(&chunk_at.to_string_lossy().into_owned())?); // Convert PathBuf to String
        }

        Err(ApiError::NoSuchChunk(uid))
    }

    pub fn check_chunk_by_uid(&self, uid: Uid) -> bool {
        if let Some(chunk_at) = self.get_uid_path( // Changed Ok to Some
            &self.root_dir,
            CHUNK_DIR_NAME,
            uid,
            Some("chunk"),
        ).ok() { // Added .ok() to convert Result to Option
            exists(&chunk_at)
        }

        else {
            false
        }
    }

    pub fn get_tfidf_by_chunk_uid(
        &self,
        uid: Uid,
    ) -> Result<ProcessedDoc, ApiError> {
        let tfidf_at = self.get_uid_path(
            &self.root_dir,
            CHUNK_DIR_NAME,
            uid,
            Some("tfidf"),
        )?;

        if exists(&tfidf_at) {
            return Ok(tfidf::load_from_file(&tfidf_at.to_string_lossy().into_owned())?); // Convert PathBuf to String
        }

        Err(ApiError::NoSuchChunk(uid))
    }

    pub fn get_tfidf_by_file_uid(
        &self,
        uid: Uid,
    ) -> Result<ProcessedDoc, ApiError> {
        let chunk_uids = self.get_chunks_of_file(uid)?;
        let mut result = ProcessedDoc::empty();

        for uid in chunk_uids.iter() {
            result.extend(&self.get_tfidf_by_chunk_uid(*uid)?);
        }

        result.uid = Some(uid);
        Ok(result)
    }

    pub fn get_chunks_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
        let file_index_path = self.get_uid_path(
            &self.root_dir,
            FILE_INDEX_DIR_NAME,
            file_uid,
            None,
        )?;

        if exists(&file_index_path) {
            return Ok(load_from_file(&file_index_path.to_string_lossy().into_owned())?); // Convert PathBuf to String
        }

        Err(ApiError::NoSuchFile { path: None, uid: Some(file_uid) })
    }

    pub fn get_merged_chunk_of_file(&self, file_uid: Uid) -> Result<RenderedChunk, ApiError> {
        let chunk_uids = self.get_chunks_of_file(file_uid)?;
        let mut chunks = Vec::with_capacity(chunk_uids.len());

        for chunk_uid in chunk_uids.iter() {
            chunks.push(self.get_chunk_by_uid(*chunk_uid)?);
        }

        // FIXME: I don't think we have to sort this
        chunks.sort_by_key(|chunk| chunk.sortable_string()); // Changed sort_by_by_key to sort_by_key
        let chunks = merge_and_convert_chunks(self, chunks)?;

        match chunks.len() {
            0 => todo!(),  // It's an empty file. Does ragit create a chunk for an empty file? I don't remember...
            1 => Ok(chunks[0].clone()),
            _ => Err(ApiError::BrokenIndex(format!("Internal error: `get_merged_chunk_of_file({file_uid})` returned multiple chunks"))),
        }
    }

    pub fn get_images_of_file(&self, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
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

    pub fn get_image_bytes_by_uid(&self, uid: Uid) -> Result<Vec<u8>, ApiError> {
        Ok(read_bytes(&self.get_uid_path(IMAGE_DIR_NAME, uid, Some("png"))?.to_string_lossy().into_owned())?) // Convert PathBuf to String
    }

    pub fn get_image_description_by_uid(&self, uid: Uid) -> Result<ImageSchema, ApiError> {
        let j = read_string(&self.get_uid_path(IMAGE_DIR_NAME, uid, Some("json"))?.to_string_lossy().into_owned())?; // Convert PathBuf to String
        let v = serde_json::from_str::<ImageSchema>(&j)?;
        Ok(v)
    }
}
