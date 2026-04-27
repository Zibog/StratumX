#![warn(unused_imports)]
#![warn(unused_variables)]
#![warn(dead_code)]

// New modular exports - make modules public
pub mod destruction_response;
pub mod fire_weather;
pub mod hydrology;
pub mod storm_celestial;
pub mod terrain_response;
mod material_registry;
mod material_types;
mod production_archetypes;

// Re-export all modules
pub use material_registry::*;
pub use material_types::*;
pub use production_archetypes::*;

// Direct re-exports from submodules for backward compatibility
pub use destruction_response::{DestructionResponse, FailureMode, Fragment, StructureType};
pub use fire_weather::{CombustibleMaterial, CombustibleObject, FireState, SmokeParticle, SmokeSystem, WetnessState};
pub use hydrology::{Evaporation, FluidContainer, HydrologyState, Leak, Rainfall};
pub use terrain_response::{CraterMorphology, DebrisParticle, TerrainBlastResponse, TerrainMaterialType};

// Legacy re-exports for backward compatibility
pub use storm_celestial::{
    AtmosphereProfileState, CelestialTimeState, CloudProfileState, CloudShadowProjectorState,
    FogState, ShadowMapPosture, SkyWeatherState, StormFrontState, SunProfileState,
    WeatherCellState, WeatherDirectorState, WeatherDistanceTier, WeatherRegime,
};

pub use material_types::{
    AcousticSurfaceClass, BuoyancyResponse, DamageMemory, FireResponse, FractureMode,
    ImpactResponseInput, ImpactResponseSummary, LayerDamageState, LayerImpactResponse,
    MaterialArchetype, MaterialArchetypeId, MaterialBehaviorFlags, MaterialConfig,
    MaterialDescriptor, MaterialId, MaterialLayer, MaterialLookupResult, MaterialStack,
    MaterialStackId, MechanicalClass, PropertyDomain, ReactionRow, ResponseProfileId, SegmentState,
    SegmentationMode, StructuralResponse,
};
