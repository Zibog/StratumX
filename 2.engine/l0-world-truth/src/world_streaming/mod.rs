// World Streaming - модульная структура

pub mod eviction;
pub mod mapping;
pub mod residency;
pub mod state;

pub use mapping::{world_pos_to_region_key, RegionKey, REGION_SIZE};
pub use residency::{MemoryPressure, ResidencyState};
pub use state::{EvictionPolicy, RegionMetadata, StreamingManager};
