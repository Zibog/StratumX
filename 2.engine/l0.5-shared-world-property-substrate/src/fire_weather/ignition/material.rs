use crate::{BurnResponseFamily, MaterialId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombustibleMaterial {
    Wood,
    Grass,
    Paper,
    Cloth,
    Plastic,
}

impl CombustibleMaterial {
    pub fn default_material_id(&self) -> MaterialId {
        match self {
            Self::Wood => MaterialId(1),
            Self::Grass => MaterialId(2),
            Self::Paper => MaterialId(3),
            Self::Cloth => MaterialId(4),
            Self::Plastic => MaterialId(5),
        }
    }

    pub fn burn_response_family(&self) -> BurnResponseFamily {
        match self {
            Self::Wood => BurnResponseFamily::SlowBurn,
            Self::Grass | Self::Paper => BurnResponseFamily::FastBurn,
            Self::Cloth => BurnResponseFamily::Smoldering,
            Self::Plastic => BurnResponseFamily::Explosive,
        }
    }

    pub fn ignition_temperature_celsius(&self) -> f32 {
        match self {
            Self::Wood => 300.0,
            Self::Grass => 250.0,
            Self::Paper => 230.0,
            Self::Cloth => 260.0,
            Self::Plastic => 350.0,
        }
    }

    pub fn wetness_ignition_threshold(&self) -> f32 {
        match self {
            Self::Wood => 30.0,
            Self::Grass => 40.0,
            Self::Paper => 20.0,
            Self::Cloth => 35.0,
            Self::Plastic => 10.0,
        }
    }

    pub fn drying_rate_percent_per_sec(&self, temperature: f32) -> f32 {
        let base_rate = match self {
            Self::Wood => 0.5,
            Self::Grass => 2.0,
            Self::Paper => 3.0,
            Self::Cloth => 1.5,
            Self::Plastic => 0.1,
        };
        base_rate * (temperature / 20.0).max(1.0)
    }
}
