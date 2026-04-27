// Viewport Frame Delivery

pub mod frame_delivery;
pub mod frame_projection;
pub mod frame_state;

use editor_dto_law::{
    BudgetPosture, CameraState, DegradedReason, FailureClass, FidelityClass, RuntimeEntryRef,
    SkyEnvironmentBindingRef, StableWorldId, TerrainBindingRef, ViewportBootFrame,
    ViewportDegradedFrame, ViewportFailureProjection, ViewportFrameDto, ViewportId,
    ViewportLiveFrame, ViewportPlayAttachedFrame, ViewportStatsDto, WeatherStateSnapshot,
};

use frame_delivery::FrameDeliveryState;
pub use frame_state::ViewportFrame;

pub struct ViewportFrameDelivery {
    delivery: FrameDeliveryState,
    stats: ViewportStatsDto,
}

impl ViewportFrameDelivery {
    pub fn new() -> Self {
        Self {
            delivery: FrameDeliveryState::new(),
            stats: ViewportStatsDto {
                fps: 0.0,
                frame_time_ms: 0.0,
                budget_posture: BudgetPosture::Healthy,
                terrain_posture: "Unbound".to_string(),
                environment_posture: "Unbound".to_string(),
                runtime_posture: "Inactive".to_string(),
                weather_posture: "Unknown".to_string(),
                cloud_shadow_posture: "Unknown".to_string(),
            },
        }
    }

    pub fn deliver_boot_frame(
        &mut self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
    ) -> ViewportBootFrame {
        self.delivery.deliver_boot_frame(world_ref, viewport_id)
    }

    pub fn deliver_live_frame(
        &mut self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
        terrain_ref: Option<TerrainBindingRef>,
        environment_ref: Option<SkyEnvironmentBindingRef>,
        weather_state: Option<WeatherStateSnapshot>,
    ) -> ViewportLiveFrame {
        if let Some(ref weather) = weather_state {
            self.stats.weather_posture = weather.weather_regime.clone();
            self.stats.cloud_shadow_posture = if weather.cloud_shadow_active {
                "Active".to_string()
            } else {
                "Disabled".to_string()
            };
        }

        self.delivery.deliver_live_frame(
            world_ref,
            viewport_id,
            terrain_ref,
            environment_ref,
            weather_state,
        )
    }

    pub fn deliver_degraded_frame(
        &mut self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
        reason: DegradedReason,
        fidelity: FidelityClass,
    ) -> ViewportDegradedFrame {
        self.delivery
            .deliver_degraded_frame(world_ref, viewport_id, reason, fidelity)
    }

    pub fn deliver_failure_projection(
        &mut self,
        viewport_id: ViewportId,
        failure: FailureClass,
        recovery: Vec<String>,
    ) -> ViewportFailureProjection {
        self.delivery
            .deliver_failure_projection(viewport_id, failure, recovery)
    }

    pub fn deliver_play_attached_frame(
        &mut self,
        runtime_ref: RuntimeEntryRef,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
        camera: CameraState,
    ) -> ViewportPlayAttachedFrame {
        self.delivery
            .deliver_play_attached_frame(runtime_ref, world_ref, viewport_id, camera)
    }

    pub fn get_frame_dto(
        &self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
    ) -> ViewportFrameDto {
        frame_projection::project_frame_to_dto(&self.delivery.current_frame, world_ref, viewport_id)
    }

    pub fn update_stats(&mut self, fps: f32, frame_time_ms: f32) {
        self.stats.fps = fps;
        self.stats.frame_time_ms = frame_time_ms;

        self.stats.budget_posture = if frame_time_ms < 16.0 {
            BudgetPosture::Healthy
        } else if frame_time_ms < 33.0 {
            BudgetPosture::Warning
        } else {
            BudgetPosture::Critical
        };
    }

    pub fn get_stats(&self) -> &ViewportStatsDto {
        &self.stats
    }

    pub fn get_frame_counter(&self) -> u64 {
        self.delivery.frame_counter
    }
}

impl Default for ViewportFrameDelivery {
    fn default() -> Self {
        Self::new()
    }
}
