use ragit_error::ApiError;
use lazy_static::lazy_static;
use regex::Regex;
//use ragit_index_core::index_struct::Index;
use ragit_index_core::index_struct::Index;
use ragit_uid::Uid;

//use ragit_index::Index;
//use ragit_index::error::Error;
// pub use ragit_index::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME, IMAGE_DIR_NAME};
// pub use ragit_fs::{read_dir, file_name, extension, is_dir, join3, join4, exists, get_relative_path};

lazy_static! {
    // full or prefix
    static ref UID_RE: Regex = Regex::new(r"^([0-9a-z]{1,64})$").unwrap();
}

#[derive(Clone, Debug)]
pub struct UidQueryConfig {
    pub search_chunk: bool,
    pub search_image: bool,
    pub search_file_uid: bool,
    pub search_file_path: bool,
    pub search_staged_file: bool,
}

impl UidQueryConfig {
    pub fn new() -> Self {
        Self {
            search_chunk: true,
            search_image: true,
            search_file_uid: true,
            search_file_path: true,
            search_staged_file: true,
        }
    }

    pub fn chunk_only(mut self) -> Self {
        self.search_chunk = true;
        self.search_image = false;
        self.search_file_uid = false;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn image_only(mut self) -> Self {
        self.search_chunk = false;
        self.search_image = true;
        self.search_file_uid = false;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn file_uid_only(mut self) -> Self {
        self.search_chunk = false;
        self.search_image = false;
        self.search_file_uid = true;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn file_path_only(mut self) -> Self {
        self.search_chunk = false;
        self.search_image = false;
        self.search_file_uid = false;
        self.search_file_path = true;
        self.search_staged_file = false;
        self
    }
}

#[derive(Clone, Debug)]
pub struct UidQueryResult {
    pub chunks: Vec<Uid>,
    pub images: Vec<Uid>,
    pub processed_files: Vec<(String, Uid)>,
    pub staged_files: Vec<String>,
}

impl UidQueryResult {
    pub fn empty() -> Self {
        Self {
            chunks: vec![],
            images: vec![],
            processed_files: vec![],
            staged_files: vec![],
        }
    }

    pub fn get_chunk_uids(&self) -> Vec<Uid> {
        self.chunks.clone()
    }
}

pub fn uid_query(
    _index: &Index,
    _qs: &[String],
    _config: UidQueryConfig,
) -> Result<UidQueryResult, ApiError> {
    Ok(UidQueryResult::empty())
}

// pub fn uid_query_unit(index: &Index, q: &str, config: UidQueryConfig) -> Result<UidQueryResult, Error> {
// pub fn uid_query_unit(index: &Index, q: &str, config: UidQueryConfig) -> Result<UidQueryResult, Error> {
//     if q.is_empty() {
//         return Ok(UidQueryResult::empty());
//     }

//     let mut chunks = vec![];
//     let mut images = vec![];
//     let mut staged_files = vec![];

//     // below 2 are for processed files
//     let mut file_uids = vec![];
//     let mut file_paths = vec![];

//     if UID_RE.is_match(q) {
//         if q.len() == 1 {
//             if config.search_chunk {
//                 for chunk_dir in read_dir(path_utils::str_to_path_ref(&join3(
//                     path_utils::pathbuf_to_str(&index.root_dir),
//                     INDEX_DIR_NAME,
//                     CHUNK_DIR_NAME,
//                 )?), false).unwrap_or(vec![]) {
//                     let chunk_prefix = file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_dir)))?;

//                     if chunk_prefix.starts_with(q) {
//                         for chunk_file in read_dir(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_dir)), false)? {
//                             if extension(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_file)))?.unwrap_or(String::new()) != "chunk" {
//                                 continue;
//                             }

//                             chunks.push(Uid::from_prefix_and_suffix(&chunk_prefix, &file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_file)))?)?);
//                         }
//                     }
//                 }
//             }

//             if config.search_file_uid {
//                 for file_index_dir in read_dir(path_utils::str_to_path_ref(&join3(
//                     path_utils::pathbuf_to_str(&index.root_dir),
//                     INDEX_DIR_NAME,
//                     FILE_INDEX_DIR_NAME,
//                 )?), false).unwrap_or(vec![]) {
//                     let file_index_prefix = file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&file_index_dir)))?;

//                     if file_index_prefix.starts_with(q) {
//                         for file_index in read_dir(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&file_index_dir)), false)? {
//                             file_uids.push(Uid::from_prefix_and_suffix(&file_index_prefix, &file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&file_index)))?)?);
//                         }
//                     }
//                 }
//             }

//             if config.search_image {
//                 for image_dir in read_dir(path_utils::str_to_path_ref(&join3(
//                     path_utils::pathbuf_to_str(&index.root_dir),
//                     INDEX_DIR_NAME,
//                     IMAGE_DIR_NAME,
//                 )?), false).unwrap_or(vec![]) {
//                     let image_prefix = file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_dir)))?;

//                     if image_prefix.starts_with(q) {
//                         for image_file in read_dir(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_dir)), false)? {
//                             if extension(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_file)))?.unwrap_or(String::new()) != "png" {
//                                 continue;
//                             }

//                             images.push(Uid::from_prefix_and_suffix(&image_prefix, &file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_file)))?)?);
//                         }
//                     }
//                 }
//             }
//         }

//         else if q.len() == 2 {
//             if config.search_chunk {
//                 for chunk_file in read_dir(path_utils::str_to_path_ref(&join4(
//                     path_utils::pathbuf_to_str(&index.root_dir),
//                     INDEX_DIR_NAME,
//                     CHUNK_DIR_NAME,
//                     q,
//                 )?), false).unwrap_or(vec![]) {
//                     if extension(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_file)))?.unwrap_or(String::new()) != "chunk" {
//                         continue;
//                     }

//                     chunks.push(Uid::from_prefix_and_suffix(q, &file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_file)))?)?);
//                 }
//             }

//             if config.search_file_uid {
//                 for file_index in read_dir(path_utils::str_to_path_ref(&join4(
//                     path_utils::pathbuf_to_str(&index.root_dir),
//                     INDEX_DIR_NAME,
//                     FILE_INDEX_DIR_NAME,
//                     q,
//                 )?), false).unwrap_or(vec![]) {
//                     file_uids.push(Uid::from_prefix_and_suffix(q, &file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&file_index)))?)?);
//                 }
//             }

//             if config.search_image {
//                 for image_file in read_dir(path_utils::str_to_path_ref(&join4(
//                     path_utils::pathbuf_to_str(&index.root_dir),
//                     INDEX_DIR_NAME,
//                     IMAGE_DIR_NAME,
//                     q,
//                 )?), false).unwrap_or(vec![]) {
//                     if extension(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_file)))?.unwrap_or(String::new()) != "png" {
//                         continue;
//                     }

//                     images.push(Uid::from_prefix_and_suffix(q, &file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_file)))?)?);
//                 }
//             }
//         }

//         else {
//             let prefix = q.get(0..2).unwrap().to_string();
//             let suffix = q.get(2..).unwrap().to_string();

//             if config.search_chunk {
//                 if q.len() == 64 {
//                     let chunk_at = join(
//                         path_utils::str_to_path_ref(&join3(
//                             path_utils::pathbuf_to_str(&index.root_dir),
//                             INDEX_DIR_NAME,
//                             CHUNK_DIR_NAME,
//                         )?),
//                         path_utils::str_to_path_ref(&join(
//                             &prefix,
//                             &set_extension(
//                                 &suffix,
//                                 "chunk",
//                             )?,
//                         )?),
//                     )?;

//                     if exists(path_utils::str_to_path_ref(path_utils::pathbuf_to_str(&chunk_at))) {
//                         chunks.push(q.parse::<Uid>()?);
//                     }
//                 }

//                 else {
//                     for chunk_file in read_dir(path_utils::str_to_path_ref(&join4(
//                         path_utils::pathbuf_to_str(&index.root_dir),
//                         INDEX_DIR_NAME,
//                         CHUNK_DIR_NAME,
//                         &prefix,
//                     )?), false).unwrap_or(vec![]) {
//                         if extension(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_file)))?.unwrap_or(String::new()) != "chunk" {
//                             continue;
//                         }

//                         let chunk_file = file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&chunk_file)))?;

//                         if chunk_file.starts_with(&suffix) {
//                             chunks.push(Uid::from_prefix_and_suffix(&prefix, &chunk_file)?);
//                         }
//                     }
//                 }
//             }

//             if config.search_file_uid {
//                 if q.len() == 64 {
//                     let file_index = join(
//                         path_utils::str_to_path_ref(&join3(
//                             path_utils::pathbuf_to_str(&index.root_dir),
//                             INDEX_DIR_NAME,
//                             FILE_INDEX_DIR_NAME,
//                         )?),
//                         path_utils::str_to_path_ref(&join(
//                             &prefix,
//                             &suffix,
//                         )?),
//                     )?;

//                     if exists(path_utils::str_to_path_ref(path_utils::pathbuf_to_str(&file_index))) {
//                         file_uids.push(q.parse::<Uid>()?);
//                     }
//                 }

//                 else {
//                     for file_index in read_dir(path_utils::str_to_path_ref(&join4(
//                         path_utils::pathbuf_to_str(&index.root_dir),
//                         INDEX_DIR_NAME,
//                         FILE_INDEX_DIR_NAME,
//                         &prefix,
//                     )?), false).unwrap_or(vec![]) {
//                         let file_index = file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&file_index)))?;

//                         if file_index.starts_with(&suffix) {
//                             file_uids.push(Uid::from_prefix_and_suffix(&prefix, &file_index)?);
//                         }
//                     }
//                 }
//             }

//             if config.search_image {
//                 if q.len() == 64 {
//                     let image_at = join(
//                         path_utils::str_to_path_ref(&join3(
//                             path_utils::pathbuf_to_str(&index.root_dir),
//                             INDEX_DIR_NAME,
//                             IMAGE_DIR_NAME,
//                         )?),
//                         path_utils::str_to_path_ref(&join(
//                             &prefix,
//                             &set_extension(
//                                 &suffix,
//                                 "png",
//                             )?,
//                         )?),
//                     )?;

//                     if exists(path_utils::str_to_path_ref(path_utils::pathbuf_to_str(&image_at))) {
//                         images.push(q.parse::<Uid>()?);
//                     }
//                 }

//                 else {
//                     for image_file in read_dir(path_utils::str_to_path_ref(&join4(
//                         path_utils::pathbuf_to_str(&index.root_dir),
//                         INDEX_DIR_NAME,
//                         IMAGE_DIR_NAME,
//                         &prefix,
//                     )?), false).unwrap_or(vec![]) {
//                         if extension(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_file)))?.unwrap_or(String::new()) != "png" {
//                             continue;
//                         }

//                         let image_file = file_name(path_utils::str_to_path_ref(&path_utils::pathbuf_to_str(&image_file)))?;

//                         if image_file.starts_with(&suffix) {
//                             images.push(Uid::from_prefix_and_suffix(&prefix, &image_file)?);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     if config.search_file_path {
//         if let Ok(mut rel_path) = get_relative_path(path_utils::pathbuf_to_str(&index.root_dir), q) {
//             // 1. It tries to exact-match a processed file.
//             if index.processed_files.contains_key(&path_utils::str_to_pathbuf(&rel_path)) {
//                 file_paths.push(rel_path.to_string());
//             }

//             // 2. It tries to exact-match a staged file.
//             //    In some cases, a file can be both processed and staged at the
//             //    same time. In that case, it has to choose the processed file.
//             else if config.search_staged_file && index.staged_files.contains(&path_utils::str_to_pathbuf(&rel_path)) {
//                 staged_files.push(rel_path);
//             }

//             // 3. It assumes that `rel_path` is a directory and tries to
//             //    find files in the directory.
//             else {
//                 if !rel_path.ends_with("/") && !rel_path.is_empty() {
//                     rel_path = format!("{rel_path}/");
//                 }

//                 for path in index.processed_files.keys() {
//                     if path.starts_with(&rel_path) {
//                         file_paths.push(path.to_string());
//                     }
//                 }

//                 if config.search_staged_file {
//                     for staged_file in index.staged_files.iter() {
//                         if staged_file.starts_with(&rel_path) {
//                             staged_files.push(staged_file.to_string());
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     let mut processed_files = HashSet::with_capacity(file_paths.len() + file_uids.len());
//     let processed_files_rev: HashMap<_, _> = index.processed_files.iter().map(
//         |(file, uid)| (*uid, file.to_string())
//     ).collect();

//     for path in file_paths.iter() {
//         processed_files.insert((path.clone(), *index.processed_files.get(&path_utils::str_to_pathbuf(path)).unwrap()));
//     }

//     for uid in file_uids.iter() {
//         processed_files.insert((processed_files_rev.get(uid).unwrap().to_string(), *uid));
//     }

//     let processed_files: Vec<(String, Uid)> = processed_files.into_iter().collect();

//     Ok(UidQueryResult {
//         chunks,
//         images,
//         processed_files,
//         staged_files,
//     })
// }
