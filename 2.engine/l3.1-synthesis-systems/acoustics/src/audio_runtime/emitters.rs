// Audio emitters - sources and footsteps

use super::types::{AudioSourceType, FootstepMaterial};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioSource {
    pub source_type: AudioSourceType,
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub volume: f32,
    pub range: f32,
    pub sound_id: u64,
    pub looping: bool,
    pub transient: bool,
    pub lifetime_sec: f32,
}

impl AudioSource {
    pub fn gunshot(position: [f32; 3]) -> Self {
        Self {
            source_type: AudioSourceType::Point,
            position,
            direction: [0.0, 0.0, 0.0],
            volume: 1.0,
            range: 100.0,
            sound_id: 1001,
            looping: false,
            transient: true,
            lifetime_sec: 0.5,
        }
    }

    pub fn explosion(position: [f32; 3], energy_j: f32) -> Self {
        let volume = (energy_j / 50000.0).min(1.5);
        let range = (energy_j / 1000.0).sqrt().min(200.0);

        Self {
            source_type: AudioSourceType::Point,
            position,
            direction: [0.0, 0.0, 0.0],
            volume,
            range,
            sound_id: 1002,
            looping: false,
            transient: true,
            lifetime_sec: 2.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.transient {
            self.lifetime_sec -= delta_time;
            self.lifetime_sec > 0.0
        } else {
            true
        }
    }

    pub fn volume_at_position(&self, position: [f32; 3]) -> f32 {
        let dx = position[0] - self.position[0];
        let dy = position[1] - self.position[1];
        let dz = position[2] - self.position[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        if distance > self.range {
            return 0.0;
        }

        let falloff = 1.0 / (1.0 + distance / 10.0);
        self.volume * falloff
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FootstepEvent {
    pub position: [f32; 3],
    pub material: FootstepMaterial,
    pub velocity: f32,
}

impl FootstepEvent {
    pub fn to_audio_source(&self) -> AudioSource {
        let base_volume = self.material.volume_modifier();
        let velocity_factor = (self.velocity / 5.0).min(1.5);
        let volume = base_volume * velocity_factor;

        AudioSource {
            source_type: AudioSourceType::Point,
            position: self.position,
            direction: [0.0, 0.0, 0.0],
            volume,
            range: 15.0,
            sound_id: self.material.sound_id(),
            looping: false,
            transient: true,
            lifetime_sec: 0.3,
        }
    }
}
