use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetGate {
    pub id: uuid::Uuid,
    pub asset_id: uuid::Uuid,
    pub status: GateStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateStatus {
    Pending,
    Approved,
    Rejected,
}

impl AssetGate {
    pub fn new(asset_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            asset_id,
            status: GateStatus::Pending,
        }
    }
}