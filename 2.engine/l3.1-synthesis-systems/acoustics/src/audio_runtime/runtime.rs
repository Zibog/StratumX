// Audio runtime - main runtime type

use super::emitters::{AudioSource, FootstepEvent};
use super::occlusion::Occluder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioRuntime {
    pub sources: Vec<AudioSource>,
    pub occluders: Vec<Occluder>,
    pub listener_position: [f32; 3],
    pub master_volume: f32,
}

impl AudioRuntime {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            occluders: Vec::new(),
            listener_position: [0.0, 0.0, 0.0],
            master_volume: 1.0,
        }
    }

    pub fn add_source(&mut self, source: AudioSource) {
        self.sources.push(source);
    }

    pub fn add_footstep(&mut self, event: FootstepEvent) {
        let source = event.to_audio_source();
        self.add_source(source);
    }

    pub fn add_occluder(&mut self, occluder: Occluder) {
        self.occluders.push(occluder);
    }

    pub fn set_listener_position(&mut self, position: [f32; 3]) {
        self.listener_position = position;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.sources.retain_mut(|source| source.update(delta_time));
    }

    pub fn calculate_mix(&self) -> f32 {
        let mut total_volume = 0.0;

        for source in &self.sources {
            let mut volume = source.volume_at_position(self.listener_position);

            for occluder in &self.occluders {
                let occlusion = occluder.occludes(source.position, self.listener_position);
                volume *= 1.0 - occlusion;
            }

            total_volume += volume;
        }

        (total_volume * self.master_volume).min(1.0)
    }
}

impl Default for AudioRuntime {
    fn default() -> Self {
        Self::new()
    }
}
