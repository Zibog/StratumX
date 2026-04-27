use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IkJoint {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub parent_index: Option<usize>,
    pub length: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IkTarget {
    pub position: [f32; 3],
    pub reached: bool,
    pub distance_to_target: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub time_sec: f32,
    pub joint_index: usize,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationClip {
    pub name: String,
    pub duration_sec: f32,
    pub keyframes: Vec<Keyframe>,
    pub looping: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationState {
    pub clip_name: String,
    pub current_time: f32,
    pub playing: bool,
    pub speed: f32,
}
