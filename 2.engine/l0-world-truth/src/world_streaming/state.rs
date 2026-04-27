// Streaming state - modules

mod manager;
mod metadata;

pub use manager::StreamingManager;
pub use metadata::{EvictionPolicy, RegionMetadata};

// Re-export types from sibling modules for convenience
pub use crate::world_streaming::mapping::RegionKey;
pub use crate::world_streaming::residency::{MemoryPressure, ResidencyState};
