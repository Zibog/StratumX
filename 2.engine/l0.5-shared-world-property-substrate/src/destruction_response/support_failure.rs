use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureType {
    Tree,
    WoodenWall,
    BrickWall,
    ConcreteWall,
    MetalSupport,
    GlassWindow,
    WoodenDoor,
}

impl StructureType {
    pub fn failure_energy_j(&self) -> f32 {
        match self {
            Self::Tree => 5000.0,
            Self::WoodenWall => 3000.0,
            Self::BrickWall => 15000.0,
            Self::ConcreteWall => 50000.0,
            Self::MetalSupport => 80000.0,
            Self::GlassWindow => 500.0,
            Self::WoodenDoor => 2000.0,
        }
    }
    pub fn fragment_count(&self) -> u32 {
        match self {
            Self::Tree => 5,
            Self::WoodenWall => 8,
            Self::BrickWall => 20,
            Self::ConcreteWall => 15,
            Self::MetalSupport => 3,
            Self::GlassWindow => 30,
            Self::WoodenDoor => 6,
        }
    }
    pub fn can_burn(&self) -> bool {
        matches!(self, Self::Tree | Self::WoodenWall | Self::WoodenDoor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailureMode {
    None,
    Crack,
    Fracture,
    Shatter,
    Collapse,
    Topple,
}

pub fn determine_failure_mode(
    structure_type: StructureType,
    impact_energy_j: f32,
) -> (FailureMode, bool) {
    let failure_threshold = structure_type.failure_energy_j();
    let destroyed = impact_energy_j >= failure_threshold;
    let failure_mode = if !destroyed {
        if impact_energy_j > failure_threshold * 0.5 {
            FailureMode::Crack
        } else {
            FailureMode::None
        }
    } else {
        match structure_type {
            StructureType::Tree => FailureMode::Topple,
            StructureType::WoodenWall | StructureType::WoodenDoor => FailureMode::Fracture,
            StructureType::BrickWall | StructureType::ConcreteWall => FailureMode::Collapse,
            StructureType::MetalSupport => FailureMode::Fracture,
            StructureType::GlassWindow => FailureMode::Shatter,
        }
    };
    (failure_mode, destroyed)
}
