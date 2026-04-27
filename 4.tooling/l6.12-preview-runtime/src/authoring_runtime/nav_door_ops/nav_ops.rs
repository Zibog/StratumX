// Navigation Operations

use crate::VerticalSliceSession;
use link_egress_observations::EditorAuthoringObservation;

pub fn handle_set_navigation_path(
    vs: &mut VerticalSliceSession,
    start: [f32; 3],
    destination: [f32; 3],
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.set_navigation_path(start, destination);
    Ok(EditorAuthoringObservation::NavigationPathSet { start, destination })
}

pub fn handle_get_navigation_status(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let path = vs.runtime.navigation_path.clone();
    vs.runtime.set_navigation_path(path.start, path.destination);
    let (status, blocked_reason) = vs.runtime.get_navigation_status();
    Ok(EditorAuthoringObservation::NavigationStatusInfo {
        status: format!("{:?}", status),
        blocked_reason,
    })
}
