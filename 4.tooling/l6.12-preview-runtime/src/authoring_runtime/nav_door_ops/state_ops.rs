// State Save/Load Operations

use crate::VerticalSliceSession;
use link_egress_observations::EditorAuthoringObservation;

pub fn handle_save_proof_scene_state(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let state_json = serde_json::to_string(&vs.runtime.save_proof_scene_state())
        .map_err(|err| format!("Failed to serialize proof scene state: {err}"))?;
    Ok(EditorAuthoringObservation::ProofSceneStateSaved { state_json })
}

pub fn handle_load_proof_scene_state(
    vs: &mut VerticalSliceSession,
    state_json: String,
) -> Result<EditorAuthoringObservation, String> {
    let state = serde_json::from_str(&state_json)
        .map_err(|err| format!("Failed to deserialize proof scene state: {err}"))?;
    vs.runtime.load_proof_scene_state(state);
    Ok(EditorAuthoringObservation::ProofSceneStateLoaded)
}

pub fn handle_reset_proof_scene_baseline(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    vs.runtime.reset_proof_scene_baseline();
    Ok(EditorAuthoringObservation::ProofSceneBaselineReset)
}

pub fn handle_save_full_world_state(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let state = vs.runtime.save_full_world_state();
    let world_version = state.world_version;
    let save_timestamp = state.save_timestamp;
    let state_json = serde_json::to_string(&state)
        .map_err(|err| format!("Failed to serialize full world state: {err}"))?;
    Ok(EditorAuthoringObservation::FullWorldStateSaved {
        state_json,
        world_version,
        save_timestamp,
    })
}

pub fn handle_load_full_world_state(
    vs: &mut VerticalSliceSession,
    state_json: String,
) -> Result<EditorAuthoringObservation, String> {
    let state: FullWorldSaveState = serde_json::from_str(&state_json)
        .map_err(|err| format!("Failed to deserialize full world state: {err}"))?;
    let world_version = state.world_version;
    vs.runtime.load_full_world_state(state)?;
    Ok(EditorAuthoringObservation::FullWorldStateLoaded { world_version })
}

pub fn handle_get_world_state_metadata(
    vs: &mut VerticalSliceSession,
) -> Result<EditorAuthoringObservation, String> {
    let state = vs.runtime.save_full_world_state();
    Ok(EditorAuthoringObservation::WorldStateMetadata {
        world_version: state.world_version,
        simulation_time: state.simulation_time,
        domains_count: count_world_domains(&state),
    })
}

// Helper function

fn count_world_domains(state: &FullWorldSaveState) -> usize {
    let mut count = 4;
    if state.npc_profile.is_some() {
        count += 1;
    }
    if state.squad.is_some() {
        count += 1;
    }
    if state.creature_ecology.is_some() {
        count += 1;
    }
    count
}
