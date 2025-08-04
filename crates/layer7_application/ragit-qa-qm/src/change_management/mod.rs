use anyhow::{Result, anyhow};
use std::fs;
use std::path::PathBuf;
use crate::definitions::{Change, ChangeStatus};
use chrono::Utc;

const CHANGES_FILE: &str = ".ragit/changes.json";

fn get_changes_file_path() -> Result<PathBuf> {
    let root_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit"); // Assuming project root
    Ok(root_dir.join(CHANGES_FILE))
}

fn load_changes() -> Result<Vec<Change>> {
    let path = get_changes_file_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let contents = fs::read_to_string(path)?;
    serde_json::from_str(&contents).map_err(|e| anyhow!("Failed to parse changes.json: {}", e))
}

fn save_changes(changes: &[Change]) -> Result<()> {
    let path = get_changes_file_path()?;
    let contents = serde_json::to_string_pretty(changes)?;
    fs::write(path, contents).map_err(|e| anyhow!("Failed to write changes.json: {}", e))
}

pub fn create_change(description: String, requested_by: String) -> Result<Change> {
    let mut changes = load_changes()?;
    let new_change = Change {
        id: uuid::Uuid::new_v4().to_string(),
        description,
        status: ChangeStatus::Requested,
        request_date: Utc::now(),
        approval_date: None,
        implementation_date: None,
        verification_date: None,
        closure_date: None,
        requested_by,
        approved_by: None,
        implemented_by: None,
        verified_by: None,
        commit_id: None,
        compliance_notes: None,
    };
    changes.push(new_change.clone());
    save_changes(&changes)?;
    Ok(new_change)
}

pub fn update_change_status(change_id: &str, new_status: ChangeStatus, updated_by: String, commit_id: Option<String>, compliance_notes: Option<String>) -> Result<Change> {
    let mut changes = load_changes()?;
    let change_index = changes.iter().position(|c| c.id == change_id)
        .ok_or_else(|| anyhow!("Change with ID {} not found", change_id))?;
    
    let mut change = changes[change_index].clone();
    change.status = new_status.clone();

    match new_status {
        ChangeStatus::Approved => {
            change.approval_date = Some(Utc::now());
            change.approved_by = Some(updated_by);
        },
        ChangeStatus::Implemented => {
            change.implementation_date = Some(Utc::now());
            change.implemented_by = Some(updated_by);
            change.commit_id = commit_id;
        },
        ChangeStatus::Verified => {
            change.verification_date = Some(Utc::now());
            change.verified_by = Some(updated_by);
        },
        ChangeStatus::Closed => {
            change.closure_date = Some(Utc::now());
        },
        _ => (),
    }
    change.compliance_notes = compliance_notes;

    changes[change_index] = change.clone();
    save_changes(&changes)?;
    Ok(change)
}

pub fn get_change(change_id: &str) -> Result<Change> {
    let changes = load_changes()?;
    changes.into_iter().find(|c| c.id == change_id)
        .ok_or_else(|| anyhow!("Change with ID {} not found", change_id))
}

pub fn list_changes() -> Result<Vec<Change>> {
    load_changes()
}
