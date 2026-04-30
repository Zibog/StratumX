//! l0.5-shared-world-property-substrate: Unified world property field substrate
//!
//! **Owner**: engine_material — canonical source of truth for world properties
//! including wetness, heat, smoke density, toxic contamination, wind hints,
//! visibility obscuration, sound pressure hints, and anomaly intensity.
//!
//! ## Crate Invariants
//! - Property fields are stored in typed storage: cell, object-local, surface, volume
//! - Update order graph defines property dependencies and prevents cycles
//! - Conflict resolution rules handle overlapping field updates
//! - Persistence codecs support full and partial-resume save/load
//!
//! ## Property Types
//! - **Wetness**: Water saturation (0.0 = dry, 1.0 = saturated)
//! - **Heat**: Temperature in Kelvin
//! - **SmokeDensity**: Smoke particle density (0.0 = clear, 1.0 = opaque)
//! - **ToxicContamination**: Toxicity level (0.0 = safe, 1.0 = lethal)
//! - **WindHint**: Wind direction and strength for simulation
//! - **VisibilityObscuration**: Visibility reduction (0.0 = clear, 1.0 = zero visibility)
//! - **SoundPressureHint**: Sound pressure level for audio propagation
//! - **AnomalyIntensity**: Special effects intensity (0.0 = normal, 1.0 = maximum)
//!
//! ## Storage Types
//! - **CellField**: Voxel-based 3D grid storage
//! - **ObjectLocalField**: Per-entity property storage
//! - **SurfaceField**: Per-surface/mesh property storage
//! - **VolumeField**: 3D region property storage
//!
//! ## Update Order
//! Properties are updated in topological order based on dependencies:
//! - Heat affects Wetness (evaporation)
//! - Wind affects SmokeDensity (dispersion)
//! - SmokeDensity affects VisibilityObscuration
//! - ToxicContamination affects AnomalyIntensity
//!
//! ## Persistence
//! - Full save/load: Serialize entire substrate
//! - Partial resume: Save only changed properties for efficient checkpointing
//! - Round-trip guarantee: load(save(x)) == x

mod exports;
pub mod queries;
pub mod runtime;
pub mod types;
pub mod validation;

// Material type submodules
mod destruction_response;
pub mod fire_weather;
mod hydrology;
mod material_response;
mod material_runtime;
mod material_types;
mod storm_celestial;
mod terrain_response;

// Registry and production types
mod material_registry;
mod production_archetypes;

pub use exports::*;
