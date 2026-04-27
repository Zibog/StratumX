//! Domain-specific compatibility types
//!
//! Defines domain version markers and all 18 SDK domain families.

use serde::{Deserialize, Serialize};

use super::types::BridgeVersion;

/// Domain-specific compatibility version markers for all 18 domain families.
///
/// Each domain carries its own version id and minimum required bridge version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainVersionMarker {
    pub domain: CompatDomain,
    pub min_bridge_version: BridgeVersion,
    pub schema_generation: u64,
}

/// All 18 SDK domain families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompatDomain {
    World,
    Terrain,
    Material,
    Destruction,
    Weather,
    Audio,
    Animation,
    Living,
    Tactics,
    Society,
    Ecology,
    Wounds,
    Photoreal,
    Diagnostics,
    Runtime,
    Build,
    Capture,
    Proof,
}

impl CompatDomain {
    pub const ALL: &'static [Self] = &[
        Self::World,
        Self::Terrain,
        Self::Material,
        Self::Destruction,
        Self::Weather,
        Self::Audio,
        Self::Animation,
        Self::Living,
        Self::Tactics,
        Self::Society,
        Self::Ecology,
        Self::Wounds,
        Self::Photoreal,
        Self::Diagnostics,
        Self::Runtime,
        Self::Build,
        Self::Capture,
        Self::Proof,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Terrain => "terrain",
            Self::Material => "material",
            Self::Destruction => "destruction",
            Self::Weather => "weather",
            Self::Audio => "audio",
            Self::Animation => "animation",
            Self::Living => "living",
            Self::Tactics => "tactics",
            Self::Society => "society",
            Self::Ecology => "ecology",
            Self::Wounds => "wounds",
            Self::Photoreal => "photoreal",
            Self::Diagnostics => "diagnostics",
            Self::Runtime => "runtime",
            Self::Build => "build",
            Self::Capture => "capture",
            Self::Proof => "proof",
        }
    }
}
