use engine_core::EngineCoreError;
use serde::{Deserialize, Serialize};
use std::fmt;

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

/// Failure reasons for animation operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationFailureReason {
    MissingSkeleton,
    MissingClip,
    InvalidSampleTime,
    BudgetRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationFailure {
    reason: AnimationFailureReason,
    message: &'static str,
}

impl AnimationFailure {
    pub const fn new(reason: AnimationFailureReason, message: &'static str) -> Self {
        Self { reason, message }
    }

    pub const fn reason(self) -> AnimationFailureReason {
        self.reason
    }

    pub const fn message(self) -> &'static str {
        self.message
    }

    pub fn into_engine_core_error(self) -> EngineCoreError {
        EngineCoreError::InvalidDescriptor(self.message)
    }
}

impl From<AnimationFailure> for EngineCoreError {
    fn from(failure: AnimationFailure) -> Self {
        failure.into_engine_core_error()
    }
}

impl fmt::Display for AnimationFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}: {}", self.reason, self.message)
    }
}

impl std::error::Error for AnimationFailure {}

/// Animation quality tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationTier {
    FullPose,
    ReducedPose,
    RootMotionOnly,
    Disabled,
}

/// Receipt for animation operation with deterministic digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationReceipt {
    pub selected_tier: AnimationTier,
    pub sampled_frame: u64,
    pub deterministic_digest: u64,
}
