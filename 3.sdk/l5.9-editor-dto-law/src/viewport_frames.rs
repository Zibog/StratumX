// Viewport Frame Classes
// Frozen semantic types for viewport frame delivery

use crate::identity::{
    RuntimeEntryRef, SkyEnvironmentBindingRef, StableWorldId, TerrainBindingRef, ViewportId,
};
use crate::world_lifecycle::FailureClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FramePosture {
    Boot,
    Live,
    Degraded,
    Failure,
    PlayAttached,
    SimulateAttached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BootSurfaceKind {
    Empty,
    Bound,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportBootFrame {
    pub world_ref: StableWorldId,
    pub viewport_id: ViewportId,
    pub posture: FramePosture,
    pub surface_kind: BootSurfaceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportLiveFrame {
    pub world_ref: StableWorldId,
    pub viewport_id: ViewportId,
    pub frame_ref: u64,
    pub terrain_ref: Option<TerrainBindingRef>,
    pub environment_ref: Option<SkyEnvironmentBindingRef>,
    pub weather_state: Option<WeatherStateSnapshot>,
}

/// Weather state snapshot for viewport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherStateSnapshot {
    pub time_of_day: f32,
    pub sun_elevation_deg: f32,
    pub cloud_coverage: f32,
    pub fog_density: f32,
    pub rain_enabled: bool,
    pub weather_regime: String,
    pub cloud_shadow_active: bool,
    pub weather_cells_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DegradedReason {
    LowMemory,
    HighLatency,
    MissingAssets,
    PartialBinding,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FidelityClass {
    Full,
    Reduced,
    Minimal,
    Placeholder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportDegradedFrame {
    pub world_ref: StableWorldId,
    pub viewport_id: ViewportId,
    pub degraded_reason: DegradedReason,
    pub fidelity_class: FidelityClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportFailureProjection {
    pub viewport_id: ViewportId,
    pub failure_class: FailureClass,
    pub recovery_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraState {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub fov: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportPlayAttachedFrame {
    pub runtime_ref: RuntimeEntryRef,
    pub world_ref: StableWorldId,
    pub viewport_id: ViewportId,
    pub camera_state: CameraState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportSimulateAttachedFrame {
    pub runtime_ref: RuntimeEntryRef,
    pub world_ref: StableWorldId,
    pub viewport_id: ViewportId,
    pub camera_state: CameraState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportFrameDto {
    pub world_ref: StableWorldId,
    pub viewport_id: ViewportId,
    pub frame_posture: FramePosture,
    pub terrain_present: bool,
    pub environment_present: bool,
    pub runtime_attached: bool,
    pub failure_class: Option<FailureClass>,
    pub weather_state: Option<WeatherStateSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BudgetPosture {
    Healthy,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportStatsDto {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub budget_posture: BudgetPosture,
    pub terrain_posture: String,
    pub environment_posture: String,
    pub runtime_posture: String,
    pub weather_posture: String,
    pub cloud_shadow_posture: String,
}
