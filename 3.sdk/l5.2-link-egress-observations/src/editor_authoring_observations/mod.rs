//! Editor authoring observations module
//!
//! Narrow re-export module for controlled transition after monolith split.

pub mod actor_observations;
pub mod animation_observations;
pub mod asset_observations;
pub mod audio;
pub mod ballistics_observations;
pub mod diagnostics;
pub mod environment;
pub mod material;
pub mod material_observations;
pub mod reason_chain;
pub mod runtime;
pub mod scene;
pub mod scene_observations;
pub mod sky;
pub mod terrain;
pub mod terrain_observations;
pub mod types;
pub mod world;
pub mod world_observations;

// Re-exports for backward compatibility
pub use animation_observations::AnimationObservation;
pub use ballistics_observations::BallisticsObservation;
pub use diagnostics::{
    DiagnosticEntryDto, PreviewStateDto, ViewportStatsDto, WorldSummaryDto, WorldTreeNodeDto,
};
pub use environment::{AssetStatusDto, SkyBundleStatusDto, StormFrontDto};
pub use material::{
    AuthoringMaterialLayerDto, AuthoringMaterialStackDto, MaterialArchetypeDto, MaterialSlotDto,
};
pub use reason_chain::ReasonChainEntryDto;
pub use terrain::{AuthoringTerrainPatchDto, SurfaceRegionDto};
pub use types::EditorAuthoringObservation;
pub use world::{
    ActorDto, ActorPresetDto, AssetDto, AuthoringSceneDto, EntityDetailsDto, EntityDto,
    TransformDto,
};
