// Runtime Entry DTOs
// Frozen semantic types for play/simulate mode entry

use crate::identity::{RuntimeEntryRef, StableWorldId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeMode {
    Play,
    Simulate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraPolicy {
    FreeCam,
    PossessWalkPawn,
    FollowEntity,
    Fixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEntryRequest {
    pub world_ref: StableWorldId,
    pub mode: RuntimeMode,
    pub camera_policy: CameraPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEntryResult {
    pub accepted: bool,
    pub runtime_ref: Option<RuntimeEntryRef>,
    pub deny_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeEntryDto {
    pub accepted: bool,
    pub runtime_ref: Option<RuntimeEntryRef>,
    pub mode: RuntimeMode,
    pub world_ref: StableWorldId,
    pub deny_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnToAuthoringResult {
    pub accepted: bool,
    pub world_ref: StableWorldId,
    pub camera_policy: CameraPolicy,
    pub discarded_runtime_state: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewStateDto {
    pub session_active: bool,
    pub world_ref: Option<StableWorldId>,
    pub frame_posture: String,
    pub runtime_attached: bool,
    pub degraded: bool,
}
