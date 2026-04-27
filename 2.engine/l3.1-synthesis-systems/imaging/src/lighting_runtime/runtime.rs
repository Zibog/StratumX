use super::types::{LightSource, ShadowCaster};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightingRuntime {
    pub lights: Vec<LightSource>,
    pub shadow_casters: Vec<ShadowCaster>,
    pub ambient_color: [f32; 3],
    pub ambient_intensity: f32,
}

impl LightingRuntime {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            shadow_casters: Vec::new(),
            ambient_color: [0.2, 0.2, 0.3],
            ambient_intensity: 0.1,
        }
    }

    pub fn add_light(&mut self, light: LightSource) {
        self.lights.push(light);
    }

    pub fn add_shadow_caster(&mut self, caster: ShadowCaster) {
        self.shadow_casters.push(caster);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.lights.retain_mut(|light| light.update(delta_time));
    }

    pub fn calculate_lighting(&self, position: [f32; 3]) -> [f32; 3] {
        let mut total_light = [
            self.ambient_color[0] * self.ambient_intensity,
            self.ambient_color[1] * self.ambient_intensity,
            self.ambient_color[2] * self.ambient_intensity,
        ];

        for light in &self.lights {
            if !light.affects_position(position) {
                continue;
            }

            let mut shadowed = false;
            for caster in &self.shadow_casters {
                if caster.blocks_light(light.position, position) {
                    shadowed = true;
                    break;
                }
            }

            if !shadowed {
                let intensity = light.intensity_at_position(position);
                total_light[0] += light.color[0] * intensity;
                total_light[1] += light.color[1] * intensity;
                total_light[2] += light.color[2] * intensity;
            }
        }

        total_light
    }
}

impl Default for LightingRuntime {
    fn default() -> Self {
        Self::new()
    }
}
