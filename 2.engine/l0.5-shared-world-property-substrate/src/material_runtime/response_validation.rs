use crate::material_response::{
    validate_non_empty, ConsequenceTier, MaterialInstanceProfile, MaterialResponseProfile,
    MaterialTriggerClass, ResponseFamilyRow, SurfaceFamilyProfile,
};
use engine_core::{EngineCoreError, EngineCoreResult};

impl SurfaceFamilyProfile {
    pub fn validate(&self) -> EngineCoreResult<()> {
        validate_non_empty(
            &self.surface_family_id,
            "surface family id must be non-empty",
        )?;
        validate_non_empty(
            &self.navigation_surface_policy_ref,
            "surface family requires navigation surface policy",
        )?;
        validate_non_empty(
            &self.acoustic_surface_policy_ref,
            "surface family requires acoustic surface policy",
        )?;
        Ok(())
    }
}

impl ResponseFamilyRow {
    pub fn validate(&self) -> EngineCoreResult<()> {
        validate_non_empty(&self.family_id, "response family id must be non-empty")?;
        if self.required_trigger_classes.is_empty() || self.required_output_branches.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "response family row requires trigger and output coverage",
            ));
        }
        validate_non_empty(
            &self.persistence_posture,
            "response family row requires persistence posture",
        )?;
        validate_non_empty(
            &self.capture_bundle_family,
            "response family row requires capture bundle family",
        )?;
        validate_non_empty(
            &self.validation_gate_family,
            "response family row requires validation gate family",
        )?;
        Ok(())
    }
}

impl MaterialResponseProfile {
    pub fn validate(&self) -> EngineCoreResult<()> {
        if self.supported_trigger_classes.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "material response profile requires supported triggers",
            ));
        }
        for value in [
            &self.contact_response_family,
            &self.penetration_response_family,
            &self.blast_response_family,
            &self.burn_response_family,
            &self.wetness_response_family,
            &self.fracture_response_family,
            &self.traversal_response_family,
            &self.persistence_response_family,
            &self.compare_baseline_family,
            &self.capture_bundle_family,
            &self.certification_pack_id,
        ] {
            validate_non_empty(value, "material response profile requires canonical refs")?;
        }
        Ok(())
    }

    pub fn family_for_trigger(&self, trigger: MaterialTriggerClass) -> &str {
        match trigger {
            MaterialTriggerClass::Contact | MaterialTriggerClass::FallOrCollision => {
                &self.contact_response_family
            }
            MaterialTriggerClass::BallisticHit | MaterialTriggerClass::BallisticGraze => {
                &self.penetration_response_family
            }
            MaterialTriggerClass::BlastOverpressure => &self.blast_response_family,
            MaterialTriggerClass::ThermalContact | MaterialTriggerClass::FireExposure => {
                &self.burn_response_family
            }
            MaterialTriggerClass::WetnessContact => &self.wetness_response_family,
            MaterialTriggerClass::ToolDigOrCut
            | MaterialTriggerClass::StructuralOverload
            | MaterialTriggerClass::TimeDegradation => &self.fracture_response_family,
            MaterialTriggerClass::RestoreOrStreamIn => &self.persistence_response_family,
        }
    }
}

impl MaterialInstanceProfile {
    pub fn validate(&self) -> EngineCoreResult<()> {
        for value in [
            &self.surface_family_id,
            &self.thickness_profile_ref,
            &self.cross_section_profile_ref,
            &self.damage_mask_family_ref,
            &self.texture_stack_ref,
            &self.weather_modulation_profile_ref,
            &self.damage_visual_profile_ref,
            &self.interaction_policy_ref,
            &self.fragment_policy_ref,
            &self.degrade_policy_ref,
        ] {
            validate_non_empty(value, "material instance profile requires canonical refs")?;
        }
        Ok(())
    }
}

impl ConsequenceTier {
    pub(crate) fn rank(self) -> u8 {
        match self {
            Self::Dormant => 0,
            Self::ContactWake => 1,
            Self::LocalConsequence => 2,
            Self::DownstreamConsequence => 3,
            Self::FarEcho => 4,
        }
    }
}
