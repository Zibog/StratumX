// Ecology Command Handlers
// Deferred: re-integrate against the new engine creature_ecology API when it is available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::EcologyCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: EcologyCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        EcologyCommand::CreateCreatureEcology {
            creature_id,
            species,
            ..
        } => Ok(EditorAuthoringObservation::CreatureEcologyCreated {
            creature_id,
            species,
        }),
        EcologyCommand::SetCreatureHunger { hunger } => {
            Ok(EditorAuthoringObservation::CreatureHungerSet { hunger })
        }
        EcologyCommand::SetCreatureFear { fear } => {
            Ok(EditorAuthoringObservation::CreatureFearSet { fear })
        }
        EcologyCommand::GetCreatureState => Ok(EditorAuthoringObservation::CreatureStateInfo {
            hunger: 0.3,
            fear: 0.1,
            migrating: false,
        }),
        EcologyCommand::EvaluateCreatureMigration => {
            Ok(EditorAuthoringObservation::CreatureMigrationEvaluated {
                migration_info: None,
            })
        }
        EcologyCommand::GetCreatureMigration => {
            Ok(EditorAuthoringObservation::CreatureMigrationInfo {
                migration_state: None,
            })
        }
    }
}
