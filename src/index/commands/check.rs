use crate::index::index_struct::Index;
use crate::error::Error;
use crate::prelude::*;
use std::collections::{HashMap, HashSet};

use super::check_chunk_files;
use super::check_file_indexes;
use super::check_images;
use super::check_configs;
use super::check_initialization;
use super::check_final_validation;

impl Index {
    /// - Check A: For each chunk file,
    ///   - Check A-0: the chunk is not corrupted.
    ///   - Check A-1: the file it points to is in `self.processed_files` (if it's `ChunkSource::File`).
    /// - Check B: For each file index,
    ///   - Check B-0: its chunk uids point to a real chunk and the chunk points to this file.
    ///   - Check B-1: `self.processed_files` has an entry for the file.
    ///   - Check B-2: All the files in `self.processed_files` have an index.
    /// - Check C: Files in `self.processed_files + self.staged_files + self.curr_processing_file` are unique.
    /// - Check D: `self.chunk_count` has a correct number.
    /// - Check E: Images in chunks are in `.ragit/images`.
    ///   - If there's and image that belongs to no chunks, that's not an error. Just run `rag gc --images`.
    /// - Check F: Images in `.ragit/images` are not corrupted, and has a proper description file.
    /// - Check G: Config files are not broken.
    pub fn check(&self) -> Result<(), Error> {
        let mut images = HashMap::<Uid, bool>::new();
        let mut chunks_to_files = HashMap::with_capacity(self.chunk_count);
        let mut processed_files = HashSet::with_capacity(self.processed_files.len());
        let mut all_chunk_uids = HashSet::with_capacity(self.chunk_count);
        let mut uids_to_files = HashMap::new();
        let mut file_uid_checks = HashMap::new();
        let mut chunk_count = 0;

        check_initialization::initialize_check_data(
            self,
            &mut images,
            &mut chunks_to_files,
            &mut processed_files,
            &mut all_chunk_uids,
            &mut uids_to_files,
            &mut file_uid_checks,
        );

        check_chunk_files::check_chunk_files(
            self,
            &mut chunks_to_files,
            &mut processed_files,
            &mut all_chunk_uids,
            &mut chunk_count,
        )?;

        check_file_indexes::check_file_indexes(
            self,
            &chunks_to_files,
            &uids_to_files,
            &mut file_uid_checks,
        );

        check_images::check_images(
            self,
            &mut images,
        )?;

        check_configs::check_configs(
            self,
        )?;

        check_final_validation::check_final_validation(
            self,
            chunk_count,
        )?;

        Ok(())
    }
}
