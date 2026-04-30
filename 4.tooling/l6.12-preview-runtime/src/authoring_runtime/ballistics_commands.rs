// Ballistics Command Handlers
// Deferred: re-integrate against the new engine kinetics/ballistic_simulation API when it is available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::{
    BallisticSimulationResultDto, EditorAuthoringObservation, ImpactResultDto, ImpactVerdictDto,
    LayerImpactEventDto,
};
use link_ingress_packets::BallisticsCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: BallisticsCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        BallisticsCommand::FireActiveActor { .. } => {
            // Deferred: keep the current observation stub until the kinetic simulation bridge is restored.
            Ok(EditorAuthoringObservation::BallisticResult {
                result: BallisticSimulationResultDto {
                    impacts: vec![ImpactResultDto {
                        target_entity: 0,
                        impact_position: [0.0, 0.0, 0.0],
                        impact_velocity: [0.0, 0.0, 0.0],
                        entry_energy_j: 0.0,
                        incidence_angle_deg: 0.0,
                        layer_events: vec![LayerImpactEventDto {
                            layer_index: 0,
                            entry_energy_j: 0.0,
                            exit_energy_j: 0.0,
                            energy_absorbed_j: 0.0,
                            verdict: ImpactVerdictDto::Stopped,
                        }],
                        final_verdict: ImpactVerdictDto::Stopped,
                    }],
                },
            })
        }
    }
}
