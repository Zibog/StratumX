//! Animation clip management

use crate::keyframe::Keyframe;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Animation clip ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnimationClipId(pub Uuid);

impl AnimationClipId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Animation clip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationClip {
    pub id: AnimationClipId,
    pub name: String,
    pub duration: f32,
    pub keyframes: Vec<Keyframe>,
    pub loop_enabled: bool,
}

impl AnimationClip {
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            id: AnimationClipId::new(),
            name,
            duration,
            keyframes: Vec::new(),
            loop_enabled: false,
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    pub fn remove_keyframe(&mut self, keyframe_id: Uuid) {
        self.keyframes.retain(|k| k.id != keyframe_id);
    }

    pub fn get_keyframe_at_time(&self, time: f32) -> Option<&Keyframe> {
        self.keyframes.iter().find(|k| (k.time - time).abs() < 0.001)
    }
}
