use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub id: uuid::Uuid,
    pub approver: String,
    pub approved: bool,
    pub comment: String,
}

impl Approval {
    pub fn new(approver: String, approved: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            approver,
            approved,
            comment: String::new(),
        }
    }
}