// Reason Chain Observations - Canonical Diagnostics DTOs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonChainEntryDto {
    pub trace_id: String,
    pub timestamp: f32,
    pub subject_id: Option<u32>,
    pub category: String,
    pub summary: String,
}
