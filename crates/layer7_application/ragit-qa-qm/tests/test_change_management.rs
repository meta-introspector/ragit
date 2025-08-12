use ragit_qa_qm::change_management;
use ragit_qa_qm::definitions::{ChangeStatus};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_create_and_list_change() {
    // Ensure a clean state for the test
    let changes_file = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/.ragit/changes.json");
    if changes_file.exists() {
        fs::remove_file(&changes_file).unwrap();
    }

    let description = "Test change request from Rust test.".to_string();
    let requested_by = "Rust Tester".to_string();
    let branch_name = Some("test-branch-rust".to_string());

    let new_change = change_management::create_change(
        description.clone(),
        requested_by.clone(),
        branch_name.clone(),
    ).unwrap();

    assert_eq!(new_change.description, description);
    assert_eq!(new_change.requested_by, requested_by);
    assert_eq!(new_change.branch_name, branch_name);
    assert_eq!(new_change.status, ChangeStatus::Requested);

    let listed_changes = change_management::list_changes().unwrap();
    assert_eq!(listed_changes.len(), 1);
    assert_eq!(listed_changes[0], new_change);

    // Clean up after the test
    if changes_file.exists() {
        fs::remove_file(&changes_file).unwrap();
    }
}

#[test]
fn test_update_change_status() {
    // Ensure a clean state for the test
    let changes_file = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/.ragit/changes.json");
    if changes_file.exists() {
        fs::remove_file(&changes_file).unwrap();
    }

    let description = "Test change for status update.".to_string();
    let requested_by = "Status Tester".to_string();
    let branch_name = Some("status-branch".to_string());

    let initial_change = change_management::create_change(
        description.clone(),
        requested_by.clone(),
        branch_name.clone(),
    ).unwrap();

    // Update status to Approved
    let updated_change = change_management::update_change_status(
        &initial_change.id,
        ChangeStatus::Approved,
        "Approver".to_string(),
        None,
        Some("Approved after review.".to_string()),
    ).unwrap();

    assert_eq!(updated_change.status, ChangeStatus::Approved);
    assert!(updated_change.approval_date.is_some());
    assert_eq!(updated_change.approved_by, Some("Approver".to_string()));
    assert_eq!(updated_change.compliance_notes, Some("Approved after review.".to_string()));

    // Update status to Implemented
    let updated_change_2 = change_management::update_change_status(
        &initial_change.id,
        ChangeStatus::Implemented,
        "Implementer".to_string(),
        Some("abcdef12345".to_string()),
        None,
    ).unwrap();

    assert_eq!(updated_change_2.status, ChangeStatus::Implemented);
    assert!(updated_change_2.implementation_date.is_some());
    assert_eq!(updated_change_2.implemented_by, Some("Implementer".to_string()));
    assert_eq!(updated_change_2.commit_id, Some("abcdef12345".to_string()));

    // Clean up after the test
    if changes_file.exists() {
        fs::remove_file(&changes_file).unwrap();
    }
}
