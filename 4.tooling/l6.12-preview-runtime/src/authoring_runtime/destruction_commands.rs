// Destruction Command Handlers
// TODO: Re-implement against new engine material_world_executor/destruction_ops API when available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::DestructionCommand;

pub fn handle(
    _session: &mut EditorAuthoringSession,
    cmd: DestructionCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        DestructionCommand::SetTerrainMaterial { material_type } => {
            // TODO: Apply material to engine terrain when material_world_executor is restored
            Ok(EditorAuthoringObservation::TerrainMaterialSet { material_type })
        }
        DestructionCommand::GetTerrainMaterial => {
            Ok(EditorAuthoringObservation::TerrainMaterialInfo {
                material_type: "default".to_string(),
            })
        }
        DestructionCommand::TriggerBlast { position, energy_j } => {
            // TODO: Execute blast against engine destruction_ops
            let _ = (position, energy_j);
            Ok(EditorAuthoringObservation::BlastTriggered {
                position,
                energy_j,
            })
        }
        DestructionCommand::GetTerrainBlastResponse => {
            Ok(EditorAuthoringObservation::TerrainBlastResponseInfo {
                crater_radius_m: 2.0,
                crater_depth_m: 0.5,
                debris_count: 12,
                ejecta_volume_m3: 4.0,
            })
        }
        DestructionCommand::SetWallIntegrity { integrity } => {
            // TODO: Apply wall integrity to engine state
            Ok(EditorAuthoringObservation::WallIntegritySet { integrity })
        }
        DestructionCommand::GetWallIntegrity => Ok(EditorAuthoringObservation::WallIntegrityInfo {
            integrity: 1.0,
        }),
        DestructionCommand::GetWallDestroyedState => {
            Ok(EditorAuthoringObservation::WallDestroyedStateInfo {
                destroyed: false,
            })
        }
        DestructionCommand::SetSupportObjectType { structure_type } => {
            Ok(EditorAuthoringObservation::SupportObjectTypeSet { structure_type })
        }
        DestructionCommand::GetSupportObjectState => {
            Ok(EditorAuthoringObservation::SupportObjectStateInfo {
                structure_type: "default".to_string(),
                integrity: 1.0,
                destroyed: false,
                failure_mode: "none".to_string(),
                fragment_count: 0,
            })
        }
        DestructionCommand::ApplySupportDamage { energy_j, .. } => {
            // TODO: Apply damage through engine destruction_ops
            let _ = energy_j;
            Ok(EditorAuthoringObservation::SupportDamageApplied { energy_j })
        }
        DestructionCommand::GetDestructionSummary => {
            Ok(EditorAuthoringObservation::DestructionSummaryInfo {
                terrain_material: "default".to_string(),
                crater_radius_m: None,
                crater_depth_m: None,
                crater_debris_count: None,
                wall_integrity: 1.0,
                wall_destroyed: false,
                support_structure_type: None,
                support_integrity: None,
                support_destroyed: None,
                support_failure_mode: None,
            })
        }
        DestructionCommand::ResetDestructionState => {
            Ok(EditorAuthoringObservation::DestructionStateReset)
        }
    }
}
