use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
// use ragit_agent::file_tree::FileTree;
use ragit_types::ApiError;

pub(crate) async fn run_read_dir(_argument: &str, _index: &Index) -> Result<ActionResult, ApiError> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    // let mut argument_path = PathBuf::from(argument);
    // if !argument.ends_with("/") {
    //     argument_path.push("");
    // }

    // let mut file_tree = FileTree::root();

    // for file in index.processed_files.keys() {
    //     if file.starts_with(&argument_path) {
    //         file_tree.insert(file.strip_prefix(&argument_path).unwrap().to_str().unwrap());
    //     }
    // }

    // if file_tree.is_empty() {
    //     Ok(ActionResult::NoSuchDir {
    //         dir: argument_path.display().to_string(),
    //         similar_dirs: vec![],
    //     })
    // } else {
    //     Ok(ActionResult::ReadDir(file_tree))
    // }
}