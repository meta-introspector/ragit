// Inverted Index Implementation
// Inverted Index is still very naive and lacking many features.
//
// 1. You can only build ii from scratch. There's no incremental build.
// 2. You can only remove the entire ii. There's no removing a single file or a chunk.
// 3. If something goes wrong while an ii is building, you have to build it from scratch.

use super::index_struct::Index;
use crate::constant::{II_DIR_NAME, INDEX_DIR_NAME, INDEX_FILE_NAME};
use crate::error::Error;
use crate::index::commands::archive::erase_lines;
use crate::ragit_path_utils::{get_ii_path, join3_paths, pathbuf_to_str, str_to_pathbuf};
use ragit_fs::{exists, is_dir, parent, read_dir, remove_dir_all, try_create_dir};
use ragit_uid::{Uid, UidWriteMode};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::cmp::Ordering;
use std::collections::HashMap;

pub type Term = String;
pub type Weight = f32;
const AUTO_FLUSH: usize = 65536; // TODO: make it configurable

// It takes too long to iterate all the terms and chunks.
const CHECK_II_LIMIT: usize = 512;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum IIStatus {
    /// Initial state. There's no ii at all.
    None,

    /// ii used to be `Complete` or `Ongoing`, but there're added or removed chunks.
    Outdated,

    /// ii is built and is usable.
    Complete,

    /// ii-building is still going on. `ii-build` commands will
    /// start from this uid. `ii-build` ALWAYS processes chunks in
    /// uid order.
    Ongoing(Uid),
}

//pub fn to_uid(p: PathBuf) -> Uid {}

#[derive(Default)]
struct IIBuildState {
    total_uid: usize,
    buffer_uid: usize,
    buffer_term: usize,
    buffer_flush: usize,
}

impl Index {
    pub fn get_search_candidates(
        &self,
        terms: &HashMap<Term, Weight>,
        limit: usize,
    ) -> Result<Vec<Uid>, Error> {
        let mut result = HashMap::new();

        for (term, weight) in terms.iter() {
            let chunk_uids = self.search_ii_by_term(term)?;
            let score =
                weight * ((self.chunk_count + 1) as f32 / (chunk_uids.len() + 1) as f32).log2();

            for chunk_uid in chunk_uids.iter() {
                match result.get_mut(chunk_uid) {
                    Some(score_) => {
                        *score_ += score;
                    }
                    None => {
                        result.insert(*chunk_uid, score);
                    }
                }
            }
        }

        let mut result = result.into_iter().collect::<Vec<_>>();

        // It has to be sorted in reverse order
        result.sort_by(|(_, score_a), (_, score_b)| {
            score_b.partial_cmp(score_a).unwrap_or(Ordering::Equal)
        });

        if result.len() > limit {
            result = result[..limit].to_vec();
        }

        Ok(result.into_iter().map(|(uid, _)| uid).collect())
    }

    pub fn search_ii_by_term(&self, term: &Term) -> Result<Vec<Uid>, Error> {
        let ii_path = get_ii_path(&self.root_dir, hash(term));

        if exists(&ii_path) {
            Ok(ragit_uid::load_from_file(&ii_path)?)
        } else {
            Ok(vec![])
        }
    }

    pub fn build_ii(&mut self, quiet: bool) -> Result<(), Error> {
        match self.ii_status {
            IIStatus::None => {}
            IIStatus::Complete => {
                return Ok(());
            }
            // TODO: resuming `Ongoing` ii-build is not implemented yet
            IIStatus::Outdated | IIStatus::Ongoing(_) => {
                self.reset_ii()?;
            }
        }

        let mut buffer = HashMap::with_capacity(AUTO_FLUSH);
        let mut state = IIBuildState::default();
        let mut uid_check_point = None;
        let mut has_to_erase_lines = false;

        for (index, uid) in self.get_all_chunk_uids()?.iter().enumerate() {
            if uid_check_point.is_none() {
                uid_check_point = Some(uid);
            }

            self.update_ii_buffer(&mut buffer, *uid)?;
            state.total_uid += 1;
            state.buffer_uid += 1;
            state.buffer_term = buffer.len();

            if !quiet && index % 8 == 0 {
                self.render_ii_build_dashboard(&state, has_to_erase_lines);
                has_to_erase_lines = true;
            }

            if buffer.len() > AUTO_FLUSH {
                self.ii_status = IIStatus::Ongoing(*uid_check_point.unwrap());
                uid_check_point = None;
                self.save_to_file(self.root_dir.join(INDEX_FILE_NAME))?;

                self.flush_ii_buffer(buffer)?;
                buffer = HashMap::with_capacity(AUTO_FLUSH);
                state.buffer_uid = 0;
                state.buffer_flush += 1;
            }
        }

        if !buffer.is_empty() {
            self.flush_ii_buffer(buffer)?;
        }

        if !quiet {
            self.render_ii_build_dashboard(&state, has_to_erase_lines);
        }

        self.ii_status = IIStatus::Complete;
        self.save_to_file(self.root_dir.join(INDEX_FILE_NAME))?;
        Ok(())
    }

    pub fn reset_ii(&mut self) -> Result<(), Error> {
        let ii_path = join3_paths(
            &self.root_dir,
            &str_to_pathbuf(INDEX_DIR_NAME),
            &str_to_pathbuf(II_DIR_NAME),
        )?;

        for dir in read_dir(&pathbuf_to_str(&ii_path), false)? {
            if is_dir(&dir) {
                remove_dir_all(&dir)?;
            }
        }

        self.ii_status = IIStatus::None;
        self.save_to_file(self.root_dir.join(INDEX_FILE_NAME.to_string()))?;
        Ok(())
    }

    pub fn is_ii_built(&self) -> bool {
        self.ii_status == IIStatus::Complete
    }

    pub(crate) fn update_ii_buffer(
        &self,
        buffer: &mut HashMap<Term, Vec<Uid>>,
        uid: Uid,
    ) -> Result<(), Error> {
        let tfidf = self.get_tfidf_by_chunk_uid(uid)?;

        for term in tfidf.term_frequency.keys() {
            match buffer.get_mut::<str>(term) {
                Some(uids) => {
                    uids.push(uid);
                }
                None => {
                    buffer.insert(term.to_string(), vec![uid]);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn flush_ii_buffer(&self, buffer: HashMap<Term, Vec<Uid>>) -> Result<(), Error> {
        for (term, uids) in buffer.into_iter() {
            let term_hash = hash(&term);
            let ii_path = get_ii_path(&self.root_dir, term_hash);
            let parent_path = parent(&ii_path)?;

            if !parent_path.exists() {
                try_create_dir(&pathbuf_to_str(&parent_path))?;
            }

            let uids = if ii_path.exists() {
                let mut prev_uids = ragit_uid::load_from_file(&ii_path)?;
                prev_uids.extend(uids);
                prev_uids
            } else {
                uids
            };

            ragit_uid::save_to_file(&ii_path, &uids, UidWriteMode::Compact)?;
        }

        Ok(())
    }

    fn render_ii_build_dashboard(&self, state: &IIBuildState, has_to_erase_lines: bool) {
        if has_to_erase_lines {
            erase_lines(6);
        }

        println!("---");
        println!("building an inverted index...");
        println!("total uid: {}", state.total_uid);
        println!("buffer uid: {}", state.buffer_uid);
        println!("buffer term: {}", state.buffer_term);
        println!("buffer flush: {}", state.buffer_flush);
    }
}

fn hash(term: &Term) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(term.as_bytes());
    format!("{:064x}", hasher.finalize())
}
