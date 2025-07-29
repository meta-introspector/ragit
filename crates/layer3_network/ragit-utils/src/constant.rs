pub const API_CONFIG_FILE_NAME: &str = "api.json";
pub const ARCHIVE_DIR_NAME: &str = "archives";
pub const BUILD_CONFIG_FILE_NAME: &str = "build.json";
pub const CHUNK_DIR_NAME: &str = "chunks";
pub const CONFIG_DIR_NAME: &str = "configs";
pub const FILE_INDEX_DIR_NAME: &str = "files";
pub const II_DIR_NAME: &str = "ii";
pub const IMAGE_DIR_NAME: &str = "images";
pub const INDEX_DIR_NAME: &str = ".ragit";
pub const INDEX_FILE_NAME: &str = "index.json";
pub const LOG_DIR_NAME: &str = "logs";
pub const METADATA_FILE_NAME: &str = "meta.json";
pub const MODEL_FILE_NAME: &str = "models.json";
pub const PROMPT_DIR_NAME: &str = "prompts";
pub const QUERY_CONFIG_FILE_NAME: &str = "query.json";
pub const CORRUPTED_CHUNK_UID_MISMATCH: &str =
    "Corrupted chunk: '{}'s uid is supposed to be '{}', but is '{}'.";
pub const CHUNK_FILE_NOT_IN_PROCESSED_FILES: &str =
    "There's a chunk of '{}', but self.processed_files does not have its entry.";
pub const FILE_INDEX_NOT_IN_PROCESSED_FILES: &str = "There's a file_index for '{}', but self.processed_files does not have an entry with such hash value.";
pub const CHUNK_POINTS_TO_WRONG_FILE: &str =
    "'{}'s file name is '{}' and it has a chunk '{}'. But the chunk points to '{}'.";
pub const CHUNK_INDEX_MISMATCH: &str = "'{}'s {}th chunk uid is '{}', but the chunk's index is {}.";
pub const CHUNK_NOT_FOUND: &str = "{} has a chunk {}, but there's no such chunk in {}/{}.";
pub const FILE_HAS_NO_INDEX: &str = "'{}' doesn't have an index.";
pub const CHUNK_COUNT_MISMATCH: &str = "self.chunk_count is {}, but the actual number is {}.";
pub const IMAGE_NOT_FOUND: &str = "{}.png not found.";
pub const INVALID_IMAGE_UID: &str = "Invalid image UID: {}.";
pub const IMAGE_DESCRIPTION_MISSING: &str = "{} exists, but {} does not exist.";
pub const CONFIG_KEY_CONFLICT: &str = "Key conflict in config file '{}': '{}'.";
pub const UID_MISMATCH: &str = "self.uid is {}, but the calculated uid is {}.";

// Memory Usage Constants
pub const MEMORY_USAGE_FORMAT: &str = r"Memory Usage ({})\: System Total\: {}, System Used\: {}, Process RSS\: {} (Delta\: {})";
pub const MEMORY_USAGE_NO_PROCESS_FORMAT: &str = r"Memory Usage ({})\: System Total\: {}, System Used\: {}";
pub const DETAILED_MEMORY_USAGE_FORMAT: &str = r"  Process Physical\: {:.2} MB, Process Virtual\: {:.2} MB";
pub const DETAILED_MEMORY_USAGE_NOT_AVAILABLE: &str = r"Detailed Memory Usage ({})\: Not available";
pub const MEMORY_LIMIT_EXCEEDED_FORMAT: &str = r"Memory limit exceeded at {}\: Process RSS {} KB > Limit {} KB ({} GB)";
pub const PROCESS_LIST_HEADER: &str = "--- Process List ({}) ---";
pub const PROCESS_INFO_FORMAT: &str = r"PID\: {}, Name\: {}, CPU\: {}%, Memory\: {} KB";
pub const PROCESS_LIST_SEPARATOR: &str = "------------------------";
pub const PROCESS_SUMMARY_HEADER: &str = "\n--- Process Memory Summary ({}) ---";
pub const PROCESS_SUMMARY_TABLE_HEADER: &str = "{:<20} {:>15}";
pub const PROCESS_SUMMARY_TABLE_SEPARATOR: &str = "{:-<20} {:-<15}";
pub const PROCESS_SUMMARY_TABLE_FOOTER: &str = "-------------------------------------";
