// Frame projection to DTO

use super::frame_state::ViewportFrame;
use editor_dto_law::{FramePosture, StableWorldId, ViewportFrameDto, ViewportId};

pub fn project_frame_to_dto(
    current_frame: &Option<ViewportFrame>,
    world_ref: StableWorldId,
    viewport_id: ViewportId,
) -> ViewportFrameDto {
    let (
        frame_posture,
        terrain_present,
        environment_present,
        runtime_attached,
        failure_class,
        weather_state,
    ) = match current_frame {
        Some(ViewportFrame::Boot(_)) => (FramePosture::Boot, false, false, false, None, None),
        Some(ViewportFrame::Live(frame)) => (
            FramePosture::Live,
            frame.terrain_ref.is_some(),
            frame.environment_ref.is_some(),
            false,
            None,
            frame.weather_state.clone(),
        ),
        Some(ViewportFrame::Degraded(_)) => {
            (FramePosture::Degraded, false, false, false, None, None)
        }
        Some(ViewportFrame::Failure(frame)) => (
            FramePosture::Failure,
            false,
            false,
            false,
            Some(frame.failure_class.clone()),
            None,
        ),
        Some(ViewportFrame::PlayAttached(_)) => {
            (FramePosture::PlayAttached, true, true, true, None, None)
        }
        None => (FramePosture::Boot, false, false, false, None, None),
    };

    ViewportFrameDto {
        world_ref,
        viewport_id,
        frame_posture,
        terrain_present,
        environment_present,
        runtime_attached,
        failure_class,
        weather_state,
    }
}
