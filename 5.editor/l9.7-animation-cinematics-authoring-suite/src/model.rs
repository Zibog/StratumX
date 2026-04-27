//! Internal model types for animation cinematics authoring

use serde::{Deserialize, Serialize};

/// Editor-side animation model representing a playable animation asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationModel {
    pub name: String,
    pub duration: f32,
    pub frame_rate: f32,
    pub loop_enabled: bool,
    pub properties: AnimationProperties,
}

/// Properties associated with an animation model
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimationProperties {
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub play_rate: f32,
}

/// Editor-side cinematic model representing a sequence of shots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicModel {
    pub name: String,
    pub shots: Vec<ShotModel>,
    pub total_duration: f32,
}

/// A single shot within a cinematic sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotModel {
    pub name: String,
    pub start_time: f32,
    pub duration: f32,
    pub camera_name: Option<String>,
}

impl AnimationModel {
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            name,
            duration,
            frame_rate: 30.0,
            loop_enabled: false,
            properties: AnimationProperties::default(),
        }
    }
}

impl CinematicModel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            shots: Vec::new(),
            total_duration: 0.0,
        }
    }

    pub fn add_shot(&mut self, shot: ShotModel) {
        let end_time = shot.start_time + shot.duration;
        if end_time > self.total_duration {
            self.total_duration = end_time;
        }
        self.shots.push(shot);
        self.shots.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
    }
}
