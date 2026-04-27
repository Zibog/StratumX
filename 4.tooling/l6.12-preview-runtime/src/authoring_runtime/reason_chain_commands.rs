// Reason Chain Command Handlers

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::ReasonChainCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: ReasonChainCommand,
) -> Result<EditorAuthoringObservation, String> {
    if let Some(vs) = &mut session.vertical_slice_session {
        match cmd {
            ReasonChainCommand::InspectNpc { npc_id } => {
                let entries = vs.authoring_inspect_reason_chain_npc(npc_id)?;
                Ok(EditorAuthoringObservation::ReasonChainNpcSlice { npc_id, entries })
            }
            ReasonChainCommand::InspectScope => {
                let entries = vs.authoring_inspect_reason_chain_scope()?;
                Ok(EditorAuthoringObservation::ReasonChainScopeSlice { entries })
            }
            ReasonChainCommand::GetStats => {
                let event_count = vs.authoring_get_reason_trace_stats()?;
                Ok(EditorAuthoringObservation::ReasonTraceStats { event_count })
            }
        }
    } else {
        Err("Runtime session not initialized".into())
    }
}
