use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureClass {
    Healthy,
    Elevated,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub heap_bytes: usize,
    pub staging_bytes: usize,
    pub allocation_count: usize,
    pub pressure: PressureClass,
}

/// Signal indicating memory pressure level change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPressureSignal {
    pub previous: PressureClass,
    pub current: PressureClass,
    pub reserved_heap_bytes: usize,
    pub reserved_staging_bytes: usize,
    pub heap_usage_percent: u8,
    pub staging_usage_percent: u8,
    pub degrade_bridge: MemoryDegradeBridge,
}

/// Bridge for graceful degradation under memory pressure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryDegradeBridge {
    pub degrade_lod: bool,
    pub degrade_texture_resolution: bool,
    pub degrade_audio_quality: bool,
    pub shed_non_critical_allocations: bool,
}

impl MemoryDegradeBridge {
    /// Create a degrade bridge based on pressure level.
    pub fn from_pressure(pressure: PressureClass) -> Self {
        match pressure {
            PressureClass::Healthy => Self {
                degrade_lod: false,
                degrade_texture_resolution: false,
                degrade_audio_quality: false,
                shed_non_critical_allocations: false,
            },
            PressureClass::Elevated => Self {
                degrade_lod: true,
                degrade_texture_resolution: false,
                degrade_audio_quality: false,
                shed_non_critical_allocations: false,
            },
            PressureClass::Critical => Self {
                degrade_lod: true,
                degrade_texture_resolution: true,
                degrade_audio_quality: true,
                shed_non_critical_allocations: true,
            },
        }
    }
}
