// Storm Command Handlers
// TODO: Re-implement against new engine weather/storm API when available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::{EditorAuthoringObservation, StormFrontDto};
use link_ingress_packets::StormCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: StormCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        StormCommand::CreateStormFront {
            position,
            radius_km,
            ..
        } => Ok(EditorAuthoringObservation::StormFrontCreated {
            front_id: 1,
            position,
            radius_km,
        }),
        StormCommand::UpdateStormFront { front_id, .. } => {
            // TODO: Apply storm front updates when weather API is restored
            let _ = front_id;
            Ok(EditorAuthoringObservation::StormFrontUpdated { front_id: 1 })
        }
        StormCommand::ListStormFronts => Ok(EditorAuthoringObservation::StormFrontList {
            fronts: vec![StormFrontDto {
                front_id: 0,
                position: [0.0, 0.0, 0.0],
                velocity: [0.0, 0.0, 0.0],
                radius_km: 0.0,
                intensity: 0.0,
                rain_intensity_mm_per_hour: 0.0,
            }],
        }),
    }
}
