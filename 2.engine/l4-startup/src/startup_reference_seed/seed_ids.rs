//! Explicit seed registry identifiers.
//!
//! Per canon 45/56: no default/fallback IDs.
//! All canonical material/terrain/environment references must come from explicit seeds.

/// Canonical seed material layer identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeedMaterialLayer {
    BaseSoil,
    GrassTop,
    RockExposure,
}

impl SeedMaterialLayer {
    pub fn canonical_id(self) -> u16 {
        match self {
            Self::BaseSoil => 1,
            Self::GrassTop => 2,
            Self::RockExposure => 3,
        }
    }
}

/// Canonical seed material archetype identifiers.
/// These must match the registration order in materials_seed.rs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeedMaterialArchetype {
    CeramicTile,
    Plaster,
    Concrete,
}

impl SeedMaterialArchetype {
    pub fn canonical_id(self) -> u16 {
        match self {
            Self::CeramicTile => 1,
            Self::Plaster => 2,
            Self::Concrete => 3,
        }
    }
}

/// Canonical seed material stack identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeedMaterialStack {
    WallTilePlasterConcrete,
}

impl SeedMaterialStack {
    pub fn canonical_id(self) -> u16 {
        match self {
            Self::WallTilePlasterConcrete => 1,
        }
    }
}

/// Canonical seed response profile identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeedResponseProfile {
    Default,
}

impl SeedResponseProfile {
    pub fn canonical_id(self) -> u16 {
        match self {
            Self::Default => 1,
        }
    }
}

/// Returns the default material layer set for a new terrain patch.
pub fn default_terrain_material_layers() -> Vec<u16> {
    vec![SeedMaterialLayer::BaseSoil.canonical_id()]
}
