// Residency state and memory pressure

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResidencyState {
    Unloaded,
    Loading,
    Resident,
    Evicting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryPressure {
    Healthy,
    Elevated,
    Critical,
}
