use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waiver {
    pub waiver_id: String,
    pub pack_id: String,
    pub waiver_type: WaiverType,
    pub description: String,
    pub approved_by: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WaiverType {
    OrangeBaselineReplacement,
    MissingHardwareFloor,
    AlternativeCompareMode,
    ConditionalCertification,
}
