use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelBoundaryReceipt {
    pub model_policy_id: u64,
    pub input_digest: u64,
    pub output_digest: u64,
    pub deterministic_digest: u64,
}
