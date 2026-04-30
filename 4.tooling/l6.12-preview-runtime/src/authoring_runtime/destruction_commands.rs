use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::DestructionCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: DestructionCommand,
) -> Result<EditorAuthoringObservation, String> {
    session.ensure_runtime_session()?;

    match cmd {
        DestructionCommand::SetTerrainMaterial { material_type } => {
            session.proof_state.destruction.terrain_material = material_type.clone();
            Ok(EditorAuthoringObservation::TerrainMaterialSet { material_type })
        }
        DestructionCommand::GetTerrainMaterial => Ok(
            EditorAuthoringObservation::TerrainMaterialInfo {
                material_type: session.proof_state.destruction.terrain_material.clone(),
            },
        ),
        DestructionCommand::TriggerBlast { position, energy_j } => {
            session.proof_state.trigger_blast(position, energy_j);
            Ok(EditorAuthoringObservation::BlastTriggered { position, energy_j })
        }
        DestructionCommand::GetTerrainBlastResponse => {
            let blast = session
                .proof_state
                .destruction
                .last_blast
                .clone()
                .unwrap_or_else(|| session.proof_state.destruction.last_blast.get_or_insert_with(|| {
                    unreachable!()
                }).clone());
            Ok(EditorAuthoringObservation::TerrainBlastResponseInfo {
                crater_radius_m: blast.crater_radius_m,
                crater_depth_m: blast.crater_depth_m,
                debris_count: blast.debris_count,
                ejecta_volume_m3: blast.ejecta_volume_m3,
            })
        }
        DestructionCommand::SetWallIntegrity { integrity } => {
            session.proof_state.set_wall_integrity(integrity);
            Ok(EditorAuthoringObservation::WallIntegritySet { integrity })
        }
        DestructionCommand::GetWallIntegrity => Ok(EditorAuthoringObservation::WallIntegrityInfo {
            integrity: session.proof_state.destruction.wall_integrity,
        }),
        DestructionCommand::GetWallDestroyedState => {
            Ok(EditorAuthoringObservation::WallDestroyedStateInfo {
                destroyed: session.proof_state.wall_destroyed(),
            })
        }
        DestructionCommand::SetSupportObjectType { structure_type } => {
            session
                .proof_state
                .set_support_object_type(structure_type.clone());
            Ok(EditorAuthoringObservation::SupportObjectTypeSet { structure_type })
        }
        DestructionCommand::GetSupportObjectState => Ok(
            EditorAuthoringObservation::SupportObjectStateInfo {
                structure_type: session.proof_state.destruction.support.structure_type.clone(),
                integrity: session.proof_state.destruction.support.integrity,
                destroyed: session.proof_state.destruction.support.destroyed,
                failure_mode: session.proof_state.destruction.support.failure_mode.clone(),
                fragment_count: session.proof_state.destruction.support.fragment_count,
            },
        ),
        DestructionCommand::ApplySupportDamage { energy_j, .. } => {
            session.proof_state.apply_support_damage(energy_j);
            Ok(EditorAuthoringObservation::SupportDamageApplied { energy_j })
        }
        DestructionCommand::GetDestructionSummary => Ok(
            EditorAuthoringObservation::DestructionSummaryInfo {
                terrain_material: session.proof_state.destruction.terrain_material.clone(),
                crater_radius_m: session
                    .proof_state
                    .destruction
                    .last_blast
                    .as_ref()
                    .map(|blast| blast.crater_radius_m),
                crater_depth_m: session
                    .proof_state
                    .destruction
                    .last_blast
                    .as_ref()
                    .map(|blast| blast.crater_depth_m),
                crater_debris_count: session
                    .proof_state
                    .destruction
                    .last_blast
                    .as_ref()
                    .map(|blast| blast.debris_count),
                wall_integrity: session.proof_state.destruction.wall_integrity,
                wall_destroyed: session.proof_state.wall_destroyed(),
                support_structure_type: Some(
                    session.proof_state.destruction.support.structure_type.clone(),
                ),
                support_integrity: Some(session.proof_state.destruction.support.integrity),
                support_destroyed: Some(session.proof_state.destruction.support.destroyed),
                support_failure_mode: Some(
                    session.proof_state.destruction.support.failure_mode.clone(),
                ),
            },
        ),
        DestructionCommand::ResetDestructionState => {
            session.proof_state.reset_destruction_state();
            Ok(EditorAuthoringObservation::DestructionStateReset)
        }
    }
}
