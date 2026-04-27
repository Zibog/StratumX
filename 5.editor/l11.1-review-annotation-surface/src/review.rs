use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: uuid::Uuid,
    pub title: String,
    pub status: ReviewStatus,
    pub annotations: Vec<uuid::Uuid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Draft,
    InReview,
    Approved,
    Rejected,
}

impl Review {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            status: ReviewStatus::Draft,
            annotations: Vec::new(),
        }
    }
}