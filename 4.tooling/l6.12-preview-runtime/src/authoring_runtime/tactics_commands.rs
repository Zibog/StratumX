// Tactics Command Handlers
// Deferred: re-integrate against the new engine squad_tactics API when it is available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::TacticsCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: TacticsCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        TacticsCommand::CreateSquad {
            squad_id,
            member_ids,
            ..
        } => Ok(EditorAuthoringObservation::SquadCreated {
            squad_id,
            member_count: member_ids.len() as u32,
        }),
        TacticsCommand::SetSquadMemberCover {
            npc_id,
            cover_position,
        } => Ok(EditorAuthoringObservation::SquadMemberCoverSet {
            npc_id,
            cover_position,
        }),
        TacticsCommand::EvaluateSquadTactic { .. } => {
            Ok(EditorAuthoringObservation::SquadTacticEvaluated {
                reasons: vec!["cover available".to_string()],
            })
        }
        TacticsCommand::GetSquadTacticState => {
            Ok(EditorAuthoringObservation::SquadTacticStateInfo {
                tactic_state: "defensive".to_string(),
            })
        }
        TacticsCommand::InvalidateSquadCover { .. } => {
            Ok(EditorAuthoringObservation::SquadCoverInvalidated {
                reasons: vec!["position exposed".to_string()],
            })
        }
        TacticsCommand::CheckCoverValidity { .. } => {
            Ok(EditorAuthoringObservation::CoverValidityInfo {
                valid: true,
                reason: "adequate cover".to_string(),
            })
        }
    }
}
