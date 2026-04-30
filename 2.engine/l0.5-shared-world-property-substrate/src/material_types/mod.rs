pub mod tags;

// Re-export types from sibling modules for backward compatibility
pub use crate::destruction_response::support_failure::FailureMode;
pub use crate::destruction_response::{DestructionResponse, StructureType};
pub use crate::fire_weather::ignition::{CombustibleMaterial, CombustibleObject, FireState};
pub use crate::fire_weather::spread::SmokeSystem;
pub use crate::fire_weather::wetness::WetnessState;
pub use crate::hydrology::{FluidContainer, HydrologyState, Leak, Rainfall};
pub use crate::terrain_response::{TerrainBlastResponse, TerrainMaterialType};

pub use tags::{
    AcousticSurfaceClass, AftermathFamilyId, BuoyancyResponse, BurnAftermathPolicy,
    BurnConsequenceReceipt, BurnResponseFamily, DamageMemory, FireExposureContext, FireResponse,
    FractureMode, ImpactResponseInput, ImpactResponseSummary, LayerDamageState,
    LayerImpactResponse, MaterialArchetype, MaterialArchetypeId, MaterialBehaviorFlags,
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialLayer, MaterialLookupResult,
    MaterialStack, MaterialStackId, MechanicalClass, PersistentBurnResult, PropertyDomain,
    ReactionRow, ResponseFamilyId, ResponseProfileId, SegmentationMode, StructuralResponse,
    SurfaceFamilyId, ThermalContactContext, WetnessModifier,
};
