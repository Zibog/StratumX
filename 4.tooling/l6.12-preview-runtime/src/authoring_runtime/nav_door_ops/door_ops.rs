// Door Operations

use crate::VerticalSliceSession;
use link_egress_observations::EditorAuthoringObservation;

pub fn handle_open_door(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.open_door()?;
    Ok(EditorAuthoringObservation::DoorOpened)
}

pub fn handle_close_door(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.close_door()?;
    Ok(EditorAuthoringObservation::DoorClosed)
}

pub fn handle_set_door_blocked(
    vs: &mut VerticalSliceSession,
    reason: String,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.set_door_blocked(reason.clone());
    Ok(EditorAuthoringObservation::DoorBlocked { reason })
}

pub fn handle_set_door_locked(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.set_door_locked();
    Ok(EditorAuthoringObservation::DoorLocked)
}

pub fn handle_get_door_state(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let (state, blocked_reason) = vs.runtime.get_door_state();
    Ok(EditorAuthoringObservation::DoorStateInfo {
        state: format!("{:?}", state),
        blocked_reason,
    })
}
