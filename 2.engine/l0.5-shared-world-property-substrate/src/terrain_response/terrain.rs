use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainMaterialType {
    Dirt,
    Mud,
    Sand,
    Gravel,
    Asphalt,
    Concrete,
    Grass,
    Snow,
    Ice,
}

impl TerrainMaterialType {
    pub fn cohesion_kpa(&self) -> f32 {
        match self {
            Self::Dirt => 15.0,
            Self::Mud => 5.0,
            Self::Sand => 0.5,
            Self::Gravel => 1.0,
            Self::Asphalt => 500.0,
            Self::Concrete => 2000.0,
            Self::Grass => 20.0,
            Self::Snow => 2.0,
            Self::Ice => 100.0,
        }
    }
    pub fn friction_angle_deg(&self) -> f32 {
        match self {
            Self::Dirt => 30.0,
            Self::Mud => 15.0,
            Self::Sand => 35.0,
            Self::Gravel => 40.0,
            Self::Asphalt => 45.0,
            Self::Concrete => 50.0,
            Self::Grass => 32.0,
            Self::Snow => 20.0,
            Self::Ice => 10.0,
        }
    }
    pub fn density_kg_m3(&self) -> f32 {
        match self {
            Self::Dirt => 1600.0,
            Self::Mud => 1800.0,
            Self::Sand => 1500.0,
            Self::Gravel => 1700.0,
            Self::Asphalt => 2300.0,
            Self::Concrete => 2400.0,
            Self::Grass => 1400.0,
            Self::Snow => 300.0,
            Self::Ice => 917.0,
        }
    }
    pub fn ejecta_coefficient(&self) -> f32 {
        match self {
            Self::Dirt => 1.2,
            Self::Mud => 0.8,
            Self::Sand => 1.5,
            Self::Gravel => 1.0,
            Self::Asphalt => 0.3,
            Self::Concrete => 0.2,
            Self::Grass => 1.1,
            Self::Snow => 2.0,
            Self::Ice => 0.5,
        }
    }
}
