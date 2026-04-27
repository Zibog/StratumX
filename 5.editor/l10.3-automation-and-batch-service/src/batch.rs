use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJob {
    pub id: uuid::Uuid,
    pub tasks: Vec<uuid::Uuid>,
    pub status: JobStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl BatchJob {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            tasks: Vec::new(),
            status: JobStatus::Pending,
        }
    }
}