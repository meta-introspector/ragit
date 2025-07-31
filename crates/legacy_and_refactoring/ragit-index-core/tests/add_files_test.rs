
use ragit_index_core::add_files::add_files_command;
use ragit_index_types::load_mode::LoadMode;
use ragit_index_types::index_struct::Index;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_add_single_file() {
    let dir = tempdir().unwrap();
    let root_dir = dir.path().to_path_buf();
    let ragit_dir = root_dir.join(".ragit");
    fs::create_dir(&ragit_dir).unwrap();
    let index_path = ragit_dir.join("index.json");

    let index = Index::new(root_dir.clone());
    let index_json = serde_json::to_string(&index).unwrap();
    fs::write(&index_path, index_json).unwrap();

    // 1. Initialize a repository
    let mut index = Index::load(root_dir.clone(), LoadMode::OnlyJson).unwrap();

    // 2. Create a new file
    let file_path = index.root_dir.join("test.txt");
    fs::write(&file_path, "Hello, world!").unwrap();

    // 3. Add the file
    let files_to_add = vec![file_path.to_str().unwrap().to_string()];
    let result = add_files_command(&mut index, &files_to_add, None, false)
        .await
        .unwrap();

    assert_eq!(result.added_files, 1);

    // 4. Verify the file is in the index
    let staged_files = index.staged_files;
    assert_eq!(staged_files.len(), 1);
    assert_eq!(staged_files[0], PathBuf::from(file_path.to_str().unwrap()));
}
