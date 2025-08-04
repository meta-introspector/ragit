use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeStatus {
    Requested,
    Approved,
    Implemented,
    Verified,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub description: String,
    pub status: ChangeStatus,
    pub request_date: DateTime<Utc>,
    pub approval_date: Option<DateTime<Utc>>,
    pub implementation_date: Option<DateTime<Utc>>,
    pub verification_date: Option<DateTime<Utc>>,
    pub closure_date: Option<DateTime<Utc>>,
    pub requested_by: String,
    pub approved_by: Option<String>,
    pub implemented_by: Option<String>,
    pub verified_by: Option<String>,
    pub commit_id: Option<String>,
    pub compliance_notes: Option<String>,
}
