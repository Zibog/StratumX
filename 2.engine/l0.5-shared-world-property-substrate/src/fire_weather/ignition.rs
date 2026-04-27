use super::wetness::WetnessState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireState {
    pub burning: bool,
    pub temperature_celsius: f32,
    pub fuel_remaining_percent: f32,
    pub ignition_time: Option<f32>,
}

impl FireState {
    pub fn new() -> Self {
        Self {
            burning: false,
            temperature_celsius: 20.0,
            fuel_remaining_percent: 100.0,
            ignition_time: None,
        }
    }
    pub fn attempt_ignition(
        &mut self,
        heat_source_temp: f32,
        material_ignition_temp: f32,
        current_time: f32,
    ) -> bool {
        if self.burning {
            return false;
        }
        if heat_source_temp >= material_ignition_temp && self.fuel_remaining_percent > 0.0 {
            self.burning = true;
            self.temperature_celsius = material_ignition_temp;
            self.ignition_time = Some(current_time);
            true
        } else {
            false
        }
    }
    pub fn extinguish(&mut self) {
        self.burning = false;
        self.temperature_celsius = 20.0;
    }
    pub fn update(&mut self, delta_time: f32) {
        if self.burning {
            self.fuel_remaining_percent = (self.fuel_remaining_percent - 1.0 * delta_time).max(0.0);
            self.temperature_celsius = 800.0;
            if self.fuel_remaining_percent <= 0.0 {
                self.extinguish();
            }
        } else {
            self.temperature_celsius = (self.temperature_celsius - 50.0 * delta_time).max(20.0);
        }
    }
}

impl Default for FireState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombustibleMaterial {
    Wood,
    Grass,
    Paper,
    Cloth,
    Plastic,
}

impl CombustibleMaterial {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombustibleObject {
    pub position: [f32; 3],
    pub material_type: CombustibleMaterial,
    pub wetness: WetnessState,
    pub fire: FireState,
}

impl CombustibleObject {
    pub fn new(position: [f32; 3], material_type: CombustibleMaterial) -> Self {
        Self {
            position,
            material_type,
            wetness: WetnessState::new(),
            fire: FireState::new(),
        }
    }
    pub fn can_ignite(&self) -> bool {
        let wetness_threshold = self.material_type.wetness_ignition_threshold();
        !self.fire.burning
            && self.wetness.wetness_percent < wetness_threshold
            && self.fire.fuel_remaining_percent > 0.0
    }
    pub fn apply_heat(&mut self, heat_temp: f32, current_time: f32) -> bool {
        if !self.can_ignite() {
            return false;
        }
        let ignition_temp = self.material_type.ignition_temperature_celsius();
        let drying_rate = self.material_type.drying_rate_percent_per_sec(heat_temp);
        self.wetness.dry(drying_rate, 1.0);
        if self.can_ignite() {
            self.fire
                .attempt_ignition(heat_temp, ignition_temp, current_time)
        } else {
            false
        }
    }
    pub fn apply_rain(&mut self, intensity: f32, current_time: f32) {
        self.wetness.add_water(intensity, current_time);
        if self.fire.burning && self.wetness.wetness_percent > 50.0 {
            self.fire.extinguish();
        }
    }
    pub fn update(&mut self, delta_time: f32, ambient_temp: f32) {
        self.fire.update(delta_time);
        let drying_rate = self.material_type.drying_rate_percent_per_sec(ambient_temp);
        self.wetness.dry(drying_rate, delta_time);
    }
}
