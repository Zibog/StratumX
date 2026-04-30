use crate::{MaterialArchetypeId, MaterialStackId, ResponseProfileId};
use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConsequenceTier {
    Dormant,
    ContactWake,
    LocalConsequence,
    DownstreamConsequence,
    FarEcho,
}

impl ConsequenceTier {
    pub fn minimum_wake_tier(trigger: MaterialTriggerClass, wetness_promotes_local: bool) -> Self {
        match trigger {
            MaterialTriggerClass::BallisticHit
            | MaterialTriggerClass::BallisticGraze
            | MaterialTriggerClass::Contact
            | MaterialTriggerClass::FallOrCollision => Self::ContactWake,
            MaterialTriggerClass::BlastOverpressure
            | MaterialTriggerClass::ThermalContact
            | MaterialTriggerClass::FireExposure
            | MaterialTriggerClass::ToolDigOrCut
            | MaterialTriggerClass::StructuralOverload => Self::LocalConsequence,
            MaterialTriggerClass::WetnessContact => {
                if wetness_promotes_local {
                    Self::LocalConsequence
                } else {
                    Self::ContactWake
                }
            }
            MaterialTriggerClass::RestoreOrStreamIn => Self::FarEcho,
            MaterialTriggerClass::TimeDegradation => Self::LocalConsequence,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialTriggerClass {
    Contact,
    BallisticHit,
    BallisticGraze,
    BlastOverpressure,
    ThermalContact,
    FireExposure,
    WetnessContact,
    ToolDigOrCut,
    StructuralOverload,
    FallOrCollision,
    TimeDegradation,
    RestoreOrStreamIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialStateModifier {
    Clean,
    Wet,
    Muddy,
    Dusty,
    Frozen,
    Charred,
    Cracked,
    Fractured,
    Bloodied,
    Mossed,
    AgeWorn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerritoryFamily {
    Landscape,
    Vegetation,
    BuiltStructure,
    Prop,
    LivingSurface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseFamilyGroup {
    PhysicalContact,
    PhysicalPenetration,
    PhysicalFracture,
    PhysicalThermal,
    PhysicalHydrology,
    RuntimeTraversal,
    RuntimeCheap,
    Persistence,
    Proof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceFamilyProfile {
    pub surface_family_id: String,
    pub territory_family: TerritoryFamily,
    pub primary_archetype_ref: MaterialArchetypeId,
    pub response_profile_ref: ResponseProfileId,
    pub navigation_surface_policy_ref: String,
    pub acoustic_surface_policy_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseFamilyRow {
    pub family_id: String,
    pub family_group: ResponseFamilyGroup,
    pub required_trigger_classes: Vec<MaterialTriggerClass>,
    pub required_output_branches: Vec<String>,
    pub cheap_runtime_rung: ConsequenceTier,
    pub fallback_family_ref: String,
    pub persistence_posture: String,
    pub capture_bundle_family: String,
    pub validation_gate_family: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialResponseProfile {
    pub response_profile_id: ResponseProfileId,
    pub supported_trigger_classes: Vec<MaterialTriggerClass>,
    pub contact_response_family: String,
    pub penetration_response_family: String,
    pub blast_response_family: String,
    pub burn_response_family: String,
    pub wetness_response_family: String,
    pub fracture_response_family: String,
    pub traversal_response_family: String,
    pub persistence_response_family: String,
    pub compare_baseline_family: String,
    pub capture_bundle_family: String,
    pub certification_pack_id: String,
    pub wetness_contact_promotes_local: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialInstanceProfile {
    pub stack_id: MaterialStackId,
    pub surface_family_id: String,
    pub territory_family: TerritoryFamily,
    pub response_profile_ref: ResponseProfileId,
    pub thickness_profile_ref: String,
    pub cross_section_profile_ref: String,
    pub damage_mask_family_ref: String,
    pub texture_stack_ref: String,
    pub weather_modulation_profile_ref: String,
    pub damage_visual_profile_ref: String,
    pub interaction_policy_ref: String,
    pub fragment_policy_ref: String,
    pub degrade_policy_ref: String,
}

pub(crate) fn validate_non_empty(value: &str, label: &'static str) -> EngineCoreResult<()> {
    if value.trim().is_empty() {
        return Err(EngineCoreError::InvalidDescriptor(label));
    }
    Ok(())
}
