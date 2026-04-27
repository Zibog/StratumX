/// Controlled re-exports for world property substrate
// Core types
pub use crate::types::{
    CellField, ConflictResolution, ConflictRule, FieldStorage, FieldValue, ObjectLocalField,
    PropertyType, SurfaceField, UpdateOrderGraph, VolumeField, WorldPropertySubstrate,
};

// Runtime operations
pub use crate::runtime::{PersistenceCodec, SubstrateRuntime};

// Validation
pub use crate::validation::{
    validate_conflict_rule, validate_coordinates, validate_dimensions, validate_field_value,
    validate_update_order, SubstrateValidationError,
};

// Queries
pub use crate::queries::{PropertyQueryResult, SubstrateQuery};

// Material types (from material_types module)
pub use crate::material_types::{
    AcousticSurfaceClass, BuoyancyResponse, CombustibleMaterial, CombustibleObject,
    DestructionResponse, FailureMode, FireResponse, FireState, FluidContainer, FractureMode,
    HydrologyState, Leak, MaterialArchetype, MaterialArchetypeId, MaterialBehaviorFlags,
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialLayer, MaterialLookupResult,
    MaterialStack, MaterialStackId, MechanicalClass, PropertyDomain, Rainfall, ReactionRow,
    ResponseProfileId, SegmentationMode, SmokeSystem, StructuralResponse, StructureType,
    TerrainBlastResponse, TerrainMaterialType, WetnessState,
};

// Material registry (separate module)
pub use crate::material_registry::MaterialRegistry;

// Sky/weather state
pub use crate::storm_celestial::cloud_field::{
    CloudProfileState, CloudShadowProjectorState, ShadowMapPosture, WeatherDistanceTier,
    WeatherCellShadowData, calculate_cloud_shadow_opacity, calculate_distance_tier,
};
pub use crate::storm_celestial::fog_state::FogState;
pub use crate::storm_celestial::simulation::{SkyWeatherState, WeatherDirectorState, WeatherRegime};
pub use crate::storm_celestial::solar_cycle::{
    AtmosphereProfileState, CelestialTimeState, SunProfileState,
};
pub use crate::storm_celestial::storm_fronts::{
    StormFrontState, create_storm_front, step_storm_front, update_storm_front,
};
pub use crate::storm_celestial::weather_cells::{
    WeatherCellState, create_weather_cell, evolve_weather_cell, update_weather_cell,
};

// Production archetypes
pub use crate::production_archetypes::{ceramic_tile, concrete, plaster};
