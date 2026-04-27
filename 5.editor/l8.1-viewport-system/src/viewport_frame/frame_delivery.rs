// Frame delivery operations

use super::frame_state::ViewportFrame;
use editor_dto_law::{
    BootSurfaceKind, CameraState, DegradedReason, FailureClass, FidelityClass, FramePosture,
    RuntimeEntryRef, SkyEnvironmentBindingRef, StableWorldId, TerrainBindingRef, ViewportBootFrame,
    ViewportDegradedFrame, ViewportFailureProjection, ViewportId, ViewportLiveFrame,
    ViewportPlayAttachedFrame, WeatherStateSnapshot,
};

pub struct FrameDeliveryState {
    pub current_frame: Option<ViewportFrame>,
    pub frame_counter: u64,
}

impl Default for FrameDeliveryState {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameDeliveryState {
    pub fn new() -> Self {
        Self {
            current_frame: None,
            frame_counter: 0,
        }
    }

    pub fn deliver_boot_frame(
        &mut self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
    ) -> ViewportBootFrame {
        let frame = ViewportBootFrame {
            world_ref,
            viewport_id,
            posture: FramePosture::Boot,
            surface_kind: BootSurfaceKind::Empty,
        };

        self.current_frame = Some(ViewportFrame::Boot(frame.clone()));
        frame
    }

    pub fn deliver_live_frame(
        &mut self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
        terrain_ref: Option<TerrainBindingRef>,
        environment_ref: Option<SkyEnvironmentBindingRef>,
        weather_state: Option<WeatherStateSnapshot>,
    ) -> ViewportLiveFrame {
        self.frame_counter += 1;

        let frame = ViewportLiveFrame {
            world_ref,
            viewport_id,
            frame_ref: self.frame_counter,
            terrain_ref,
            environment_ref,
            weather_state,
        };

        self.current_frame = Some(ViewportFrame::Live(frame.clone()));
        frame
    }

    pub fn deliver_degraded_frame(
        &mut self,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
        reason: DegradedReason,
        fidelity: FidelityClass,
    ) -> ViewportDegradedFrame {
        let frame = ViewportDegradedFrame {
            world_ref,
            viewport_id,
            degraded_reason: reason,
            fidelity_class: fidelity,
        };

        self.current_frame = Some(ViewportFrame::Degraded(frame.clone()));
        frame
    }

    pub fn deliver_failure_projection(
        &mut self,
        viewport_id: ViewportId,
        failure: FailureClass,
        recovery: Vec<String>,
    ) -> ViewportFailureProjection {
        let frame = ViewportFailureProjection {
            viewport_id,
            failure_class: failure,
            recovery_actions: recovery,
        };

        self.current_frame = Some(ViewportFrame::Failure(frame.clone()));
        frame
    }

    pub fn deliver_play_attached_frame(
        &mut self,
        runtime_ref: RuntimeEntryRef,
        world_ref: StableWorldId,
        viewport_id: ViewportId,
        camera: CameraState,
    ) -> ViewportPlayAttachedFrame {
        let frame = ViewportPlayAttachedFrame {
            runtime_ref,
            world_ref,
            viewport_id,
            camera_state: camera,
        };

        self.current_frame = Some(ViewportFrame::PlayAttached(frame.clone()));
        frame
    }
}
