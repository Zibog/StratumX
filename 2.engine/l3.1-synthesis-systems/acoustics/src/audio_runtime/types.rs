// Audio types

use engine_material::MaterialId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioSourceType {
    Point,
    Ambient,
    Directional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FootstepMaterial {
    Dirt,
    Grass,
    Gravel,
    Sand,
    Mud,
    Asphalt,
    Concrete,
    Wood,
    Metal,
    Water,
    Snow,
    Ice,
}

impl FootstepMaterial {
    pub fn from_material_id(material_id: MaterialId) -> Self {
        match material_id.0 {
            1 => FootstepMaterial::Dirt,
            2 => FootstepMaterial::Grass,
            3 => FootstepMaterial::Gravel,
            4 => FootstepMaterial::Sand,
            5 => FootstepMaterial::Mud,
            6 => FootstepMaterial::Asphalt,
            7 => FootstepMaterial::Concrete,
            8 => FootstepMaterial::Wood,
            9 => FootstepMaterial::Metal,
            10 => FootstepMaterial::Water,
            11 => FootstepMaterial::Snow,
            12 => FootstepMaterial::Ice,
            _ => FootstepMaterial::Dirt,
        }
    }

    pub fn sound_id(&self) -> u64 {
        match self {
            FootstepMaterial::Dirt => 2001,
            FootstepMaterial::Grass => 2002,
            FootstepMaterial::Gravel => 2003,
            FootstepMaterial::Sand => 2004,
            FootstepMaterial::Mud => 2005,
            FootstepMaterial::Asphalt => 2006,
            FootstepMaterial::Concrete => 2007,
            FootstepMaterial::Wood => 2008,
            FootstepMaterial::Metal => 2009,
            FootstepMaterial::Water => 2010,
            FootstepMaterial::Snow => 2011,
            FootstepMaterial::Ice => 2012,
        }
    }

    pub fn volume_modifier(&self) -> f32 {
        match self {
            FootstepMaterial::Dirt => 0.6,
            FootstepMaterial::Grass => 0.4,
            FootstepMaterial::Gravel => 0.8,
            FootstepMaterial::Sand => 0.5,
            FootstepMaterial::Mud => 0.7,
            FootstepMaterial::Asphalt => 0.7,
            FootstepMaterial::Concrete => 0.9,
            FootstepMaterial::Wood => 0.8,
            FootstepMaterial::Metal => 1.0,
            FootstepMaterial::Water => 0.9,
            FootstepMaterial::Snow => 0.5,
            FootstepMaterial::Ice => 0.6,
        }
    }

    pub fn pitch_modifier(&self) -> f32 {
        match self {
            FootstepMaterial::Dirt => 1.0,
            FootstepMaterial::Grass => 0.9,
            FootstepMaterial::Gravel => 1.1,
            FootstepMaterial::Sand => 0.95,
            FootstepMaterial::Mud => 0.85,
            FootstepMaterial::Asphalt => 1.0,
            FootstepMaterial::Concrete => 1.05,
            FootstepMaterial::Wood => 1.1,
            FootstepMaterial::Metal => 1.2,
            FootstepMaterial::Water => 0.8,
            FootstepMaterial::Snow => 0.9,
            FootstepMaterial::Ice => 1.15,
        }
    }
}
