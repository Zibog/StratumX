use super::containers::FluidContainer;
use super::leaks::Leak;
use super::rainfall::Rainfall;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaporation {
    pub temperature_celsius: f32,
    pub humidity_percent: f32,
    pub surface_area_m2: f32,
}

impl Evaporation {
    pub fn new(temperature_celsius: f32, humidity_percent: f32, surface_area_m2: f32) -> Self {
        Self {
            temperature_celsius,
            humidity_percent,
            surface_area_m2,
        }
    }
    pub fn water_lost_per_sec(&self, current_volume_liters: f32) -> f32 {
        if current_volume_liters <= 0.0 {
            return 0.0;
        }
        let temp_factor = (self.temperature_celsius / 20.0).max(0.0);
        let humidity_factor = (100.0 - self.humidity_percent) / 100.0;
        let rate_m_per_sec = (0.001 / 86400.0) * temp_factor * humidity_factor;
        (self.surface_area_m2 * rate_m_per_sec * 1000.0).min(current_volume_liters)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HydrologyState {
    pub container: FluidContainer,
    pub leaks: Vec<Leak>,
    pub rainfall: Option<Rainfall>,
    pub evaporation: Evaporation,
}

impl HydrologyState {
    pub fn new(container: FluidContainer) -> Self {
        let cross_section = container.cross_section_m2;
        Self {
            container,
            leaks: Vec::new(),
            rainfall: None,
            evaporation: Evaporation::new(20.0, 50.0, cross_section),
        }
    }
    pub fn add_leak(&mut self, leak: Leak) {
        self.leaks.push(leak);
    }
    pub fn update(&mut self, delta_time_sec: f32) {
        let water_level = self.container.current_water_level_m();
        if let Some(ref rainfall) = self.rainfall {
            let added = rainfall.water_added_per_sec() * delta_time_sec;
            self.container.add_water(added);
        }
        for leak in &mut self.leaks {
            if leak.active {
                let flow_rate = leak.flow_rate_liters_per_sec(water_level);
                self.container.remove_water(flow_rate * delta_time_sec);
                if self.container.current_water_level_m() <= leak.hole_height_m {
                    leak.active = false;
                }
            }
        }
        let evaporated = self
            .evaporation
            .water_lost_per_sec(self.container.current_volume_liters)
            * delta_time_sec;
        self.container.remove_water(evaporated);
    }
}
