pub const INSTRUCTION_READ_FILE: &str = "Give me an exact path of a file that you want to read. Don't say anything other than the path of the file.";
pub const INSTRUCTION_READ_DIR: &str = "Give me an exact path of a directory that you want to read. Don't say anything other than the path of the directory.";
pub const INSTRUCTION_READ_CHUNK: &str = "Give me an exact uid of a chunk that you want to read. A uid is a hexadecimal string that uniquely identifies a chunk. Don't say anything other than the uid of the chunk.";
pub const INSTRUCTION_SEARCH_EXACT: &str = "Give me a keyword that you want to search for. It's not a pattern, just a keyword (case-sensitive). I'll use exact-text-matching to search. Don't say anything other than the keyword.";
pub const INSTRUCTION_SEARCH_TFIDF: &str = "Give me a comma-separated list of keywords that you want to search for. Don't say anything other than the keywords.";
pub const INSTRUCTION_GET_META: &str = "Below is a list of keys in the metadata. Choose a key that you want to see. Don't say anything other than the key.\n\n{}";
pub const INSTRUCTION_GET_SUMMARY: &str = "I'll give you the summary. Hold on.";
pub const INSTRUCTION_SIMPLE_RAG: &str = "Give me a simple factual question. Don't say anything other than the question.";

pub const PROMPT_READ_FILE: &str = "Read a file: if you give me the exact path of a file, I'll show you the content of the file.";
pub const PROMPT_READ_DIR: &str = "See a list of files in a directory: if you give me the exact path of a directory, I'll show you a list of the files in the directory.";
pub const PROMPT_READ_CHUNK: &str = "Read a chunk: if you know a uid of a chunk, you can get the content of the chunk.";
pub const PROMPT_SEARCH_EXACT: &str = "Search by a keyword (exact): if you give me a keyword, I'll give you a list of files that contain the exact keyword in their contents.";
pub const PROMPT_SEARCH_TFIDF: &str = "Search by keywords (tfidf): if you give me keywords, I'll give you a tfidf search result. It tries to search for files that contain any of the keywords, even though there's no exact match.";
pub const PROMPT_GET_META: &str = "Get metadata: a knowledge-base has metadata, which is a key-value store. If you give me a key of a metadata, I'll give you what value the metadata has.";
pub const PROMPT_GET_SUMMARY: &str = "Get summary of the entire knowledge-base.";
pub const PROMPT_SIMPLE_RAG: &str = "Call a simple RAG agent: if you ask a simple factual question, a RAG agent will read the files and answer your question. You can only ask a simple factual question, not complex reasoning questions.";

pub const RENDER_FILE_LONG: &str = "The file is too long to show you. Instead, I'll show you the summaries of the chunks of the file. You can get the content of the chunks with their uid.\n\n{}";
pub const RENDER_FILE_LONG_CHUNK: &str = "{}. {}\nuid: {}\nsummary: {}";
pub const RENDER_NO_SUCH_FILE: &str = "There's no such file: `{}`{}";
pub const RENDER_NO_SUCH_FILE_SIMILAR: &str = "\nThere are files with a similar name:\n\n{}";
pub const RENDER_NO_SUCH_DIR: &str = "There's no such dir: `{}`{}";
pub const RENDER_NO_SUCH_DIR_SIMILAR: &str = "\nThere are dirs with a similar name:\n\n{}";
pub const RENDER_NO_SUCH_CHUNK_INVALID_UID: &str = "{} is not a valid uid. A uid is a 9 ~ 64 characters long hexadecimal string that uniquely identifies a chunk.";
pub const RENDER_NO_SUCH_CHUNK: &str = "There's no chunk that has uid `{}`.";
pub const RENDER_CHUNK_AMBIGUOUS_CHUNK: &str = "{}. {}\nuid: {}\ntitle: {}\nsummary: {}";
pub const RENDER_CHUNK_AMBIGUOUS: &str = "There are multiple chunks whose uid starts with `{}`. Please give me a longer uid so that I can uniquely identify the chunk.\n\n{}";
pub const RENDER_CHUNK_TOO_MANY_UNLUCKY: &str = "I'm sorry, but you're very unlucky. There're {} chunks whose uid starts with `{}`. I can't help it.";
pub const RENDER_CHUNK_TOO_MANY_AMBIGUOUS: &str = "Your query `{}` is too ambiguous. There are {} chunks whose uid starts with `{}`. Please give me a longer uid so that I can uniquely identify the chunk.";
pub const RENDER_SEARCH_EXACT_NO_MATCH: &str = "There's no file that contains the keyword `{}`. Perhaps try tfidf search with the same keyword.";
pub const RENDER_SEARCH_TFIDF_NO_MATCH: &str = "There's no file that matches keywords `{}`.";
pub const RENDER_SEARCH_HEADER: &str = "This is a list of chunks that {} `{}`.";
pub const RENDER_SEARCH_EXACT_HEADER: &str = "contains the keyword";
pub const RENDER_SEARCH_TFIDF_HEADER: &str = "matches keywords";
pub const RENDER_SEARCH_RESULT: &str = "{}\n\n{}";
pub const RENDER_SEARCH_RESULT_CHUNK: &str = "{}. {}\nuid: {}\nsummary: {}";
pub const RENDER_NO_SUCH_META: &str = "There's no such key in metadata: `{}`{}";
pub const RENDER_NO_SUCH_META_SIMILAR: &str = "\nThere are similar keys:\n\n{}";
pub const RENDER_SIMPLE_RAG: &str = "{}:{}";
pub const RENDER_SIMPLE_RAG_REFERENCED_CHUNKS: &str = "\n\n---- referenced chunks ----\n{}";
pub const RENDER_SIMPLE_RAG_REFERENCED_CHUNK: &str = "{} (uid: {})";

pub const ARGUMENT_OKAY: &str = "okay";
pub const YES: &str = "yes";
pub const NO: &str = "no";
pub const FORMAT_RESULT_RENDERED: &str = "{}

{}";
