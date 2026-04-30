// Population Command Handlers
// Deferred: re-integrate against the new engine agents API when it is available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::PopulationCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: PopulationCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        PopulationCommand::CreateNpcProfile { npc_id, name, .. } => {
            Ok(EditorAuthoringObservation::NpcProfileCreated { npc_id, name })
        }
        PopulationCommand::SetNpcTraits { .. } => Ok(EditorAuthoringObservation::NpcTraitsSet),
        PopulationCommand::GetNpcTraits => Ok(EditorAuthoringObservation::NpcTraitsInfo {
            aggression: 0.5,
            greed: 0.5,
            loyalty: 0.5,
            courage: 0.5,
            discipline: 0.5,
            sociability: 0.5,
        }),
        PopulationCommand::SetNpcNeed { need_type, value } => {
            Ok(EditorAuthoringObservation::NpcNeedSet { need_type, value })
        }
        PopulationCommand::GetNpcNeed { need_type } => {
            Ok(EditorAuthoringObservation::NpcNeedInfo {
                need_type,
                value: 0.5,
            })
        }
        PopulationCommand::SetNpcActivity { activity, .. } => {
            Ok(EditorAuthoringObservation::NpcActivitySet { activity })
        }
        PopulationCommand::GetNpcSchedule => Ok(EditorAuthoringObservation::NpcScheduleInfo {
            activity: "idle".to_string(),
            start_time: 0.0,
            duration: 0.0,
            location: [0.0, 0.0, 0.0],
        }),
        PopulationCommand::IncreaseScarcity { scarcity_factor } => {
            Ok(EditorAuthoringObservation::ScarcityIncreased { scarcity_factor })
        }
        PopulationCommand::GetCrimePressure => {
            Ok(EditorAuthoringObservation::CrimePressureInfo { pressure: 0.0 })
        }
        PopulationCommand::EvaluateCrimeEscalation { .. } => {
            Ok(EditorAuthoringObservation::CrimeEscalationEvaluated { crime_type: None })
        }
        PopulationCommand::GetCriminalStatus => {
            Ok(EditorAuthoringObservation::CriminalStatusInfo {
                is_criminal: false,
                reputation: 0.0,
                wanted_level: 0,
            })
        }
        PopulationCommand::SetNpcFaction {
            faction_id,
            reputation,
        } => Ok(EditorAuthoringObservation::NpcFactionSet {
            faction_id,
            reputation,
        }),
        PopulationCommand::GetNpcFaction => {
            Ok(EditorAuthoringObservation::NpcFactionInfo { faction_info: None })
        }
    }
}
