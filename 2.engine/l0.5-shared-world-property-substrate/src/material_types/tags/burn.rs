use super::ids::{AftermathFamilyId, MaterialId};
use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};

/// Family of burn response behaviors for materials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BurnResponseFamily {
    NonCombustible,
    SlowBurn,
    FastBurn,
    Explosive,
    Smoldering,
}

impl BurnResponseFamily {
    pub fn response_suffix(self) -> &'static str {
        match self {
            Self::NonCombustible => "non_combustible",
            Self::SlowBurn => "slow_burn",
            Self::FastBurn => "fast_burn",
            Self::Explosive => "explosive",
            Self::Smoldering => "smoldering",
        }
    }
}

/// Context for fire exposure evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireExposureContext {
    pub flame_temperature_celsius: f32,
    pub exposure_duration_seconds: f32,
    pub oxygen_availability: f32,
}

impl FireExposureContext {
    pub fn validate(&self) -> EngineCoreResult<()> {
        if self.flame_temperature_celsius <= 0.0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "fire exposure flame temperature must be positive",
            ));
        }
        if self.exposure_duration_seconds <= 0.0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "fire exposure duration must be positive",
            ));
        }
        if !(0.0..=1.0).contains(&self.oxygen_availability) {
            return Err(EngineCoreError::InvalidDescriptor(
                "fire exposure oxygen availability must be within 0..=1",
            ));
        }
        Ok(())
    }
}

/// Context for thermal contact evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThermalContactContext {
    pub contact_temperature_celsius: f32,
    pub contact_area_m2: f32,
    pub contact_duration_seconds: f32,
}

/// Modifier for wetness affecting burn behavior.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WetnessModifier {
    pub wetness_percent: f32,
    pub ignition_threshold_multiplier: f32,
    pub burn_rate_multiplier: f32,
}

impl WetnessModifier {
    pub fn from_wetness(wetness_percent: f32) -> Self {
        let threshold_mult = if wetness_percent > 50.0 {
            2.0
        } else if wetness_percent > 20.0 {
            1.5
        } else {
            1.0
        };
        let burn_mult = (1.0 - wetness_percent / 100.0).max(0.1);
        Self {
            wetness_percent,
            ignition_threshold_multiplier: threshold_mult,
            burn_rate_multiplier: burn_mult,
        }
    }
}

/// Policy for burn aftermath handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BurnAftermathPolicy {
    RemainIntact,
    ConvertToAsh,
    ConvertToCharred,
    Disintegrate,
}

impl BurnAftermathPolicy {
    pub fn for_burn_family(family: BurnResponseFamily, ignited: bool) -> Self {
        if !ignited {
            return Self::RemainIntact;
        }
        match family {
            BurnResponseFamily::NonCombustible => Self::RemainIntact,
            BurnResponseFamily::SlowBurn | BurnResponseFamily::Smoldering => Self::ConvertToCharred,
            BurnResponseFamily::FastBurn => Self::ConvertToAsh,
            BurnResponseFamily::Explosive => Self::Disintegrate,
        }
    }

    pub fn aftermath_family_id(self) -> AftermathFamilyId {
        match self {
            Self::RemainIntact => AftermathFamilyId::remain_intact(),
            Self::ConvertToAsh => AftermathFamilyId::convert_to_ash(),
            Self::ConvertToCharred => AftermathFamilyId::convert_to_charred(),
            Self::Disintegrate => AftermathFamilyId::disintegrate(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistentBurnResult {
    Intact,
    Charred,
    Ash,
    Removed,
}

impl PersistentBurnResult {
    pub fn from_policy(policy: BurnAftermathPolicy) -> Self {
        match policy {
            BurnAftermathPolicy::RemainIntact => Self::Intact,
            BurnAftermathPolicy::ConvertToCharred => Self::Charred,
            BurnAftermathPolicy::ConvertToAsh => Self::Ash,
            BurnAftermathPolicy::Disintegrate => Self::Removed,
        }
    }

    pub fn aftermath_family_id(self) -> AftermathFamilyId {
        match self {
            Self::Intact => AftermathFamilyId::remain_intact(),
            Self::Charred => AftermathFamilyId::convert_to_charred(),
            Self::Ash => AftermathFamilyId::convert_to_ash(),
            Self::Removed => AftermathFamilyId::disintegrate(),
        }
    }
}

/// Receipt confirming burn consequence application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurnConsequenceReceipt {
    pub material_id: MaterialId,
    pub burn_family: BurnResponseFamily,
    pub ignited: bool,
    pub fuel_consumed_percent: f32,
    pub wetness_percent: f32,
    pub aftermath_policy: BurnAftermathPolicy,
    pub persistent_result: PersistentBurnResult,
    pub deterministic_digest: u64,
}
