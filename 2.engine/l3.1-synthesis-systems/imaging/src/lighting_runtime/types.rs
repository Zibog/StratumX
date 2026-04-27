use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightType {
    Point,
    Spot,
    Directional,
    Area,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightSource {
    pub light_type: LightType,
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub casts_shadows: bool,
    pub transient: bool,
    pub lifetime_sec: f32,
}

impl LightSource {
    pub fn muzzle_flash(position: [f32; 3], direction: [f32; 3]) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            direction,
            color: [1.0, 0.8, 0.4],
            intensity: 500.0,
            range: 10.0,
            casts_shadows: true,
            transient: true,
            lifetime_sec: 0.05,
        }
    }

    pub fn explosion(position: [f32; 3], energy_j: f32) -> Self {
        let intensity = (energy_j / 1000.0).min(2000.0);
        let range = (energy_j / 5000.0).sqrt().min(20.0);

        Self {
            light_type: LightType::Point,
            position,
            direction: [0.0, 1.0, 0.0],
            color: [1.0, 0.6, 0.2],
            intensity,
            range,
            casts_shadows: true,
            transient: true,
            lifetime_sec: 0.2,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.transient {
            self.lifetime_sec -= delta_time;
            let fade_factor = (self.lifetime_sec / 0.05).max(0.0);
            self.intensity *= fade_factor;
            self.lifetime_sec > 0.0
        } else {
            true
        }
    }

    pub fn affects_position(&self, position: [f32; 3]) -> bool {
        let dx = position[0] - self.position[0];
        let dy = position[1] - self.position[1];
        let dz = position[2] - self.position[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        distance <= self.range
    }

    pub fn intensity_at_position(&self, position: [f32; 3]) -> f32 {
        if !self.affects_position(position) {
            return 0.0;
        }

        let dx = position[0] - self.position[0];
        let dy = position[1] - self.position[1];
        let dz = position[2] - self.position[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        let falloff = 1.0 / (1.0 + distance * distance);
        self.intensity * falloff
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowCaster {
    pub position: [f32; 3],
    pub bounds: [f32; 3],
    pub casts_shadow: bool,
}

impl ShadowCaster {
    pub fn blocks_light(&self, light_pos: [f32; 3], test_pos: [f32; 3]) -> bool {
        if !self.casts_shadow {
            return false;
        }

        let to_test = [
            test_pos[0] - light_pos[0],
            test_pos[1] - light_pos[1],
            test_pos[2] - light_pos[2],
        ];

        let to_caster = [
            self.position[0] - light_pos[0],
            self.position[1] - light_pos[1],
            self.position[2] - light_pos[2],
        ];

        let dot = to_test[0] * to_caster[0] + to_test[1] * to_caster[1] + to_test[2] * to_caster[2];

        if dot > 0.0 {
            let dx = (test_pos[0] - self.position[0]).abs();
            let dy = (test_pos[1] - self.position[1]).abs();
            let dz = (test_pos[2] - self.position[2]).abs();

            dx < self.bounds[0] && dy < self.bounds[1] && dz < self.bounds[2]
        } else {
            false
        }
    }
}
