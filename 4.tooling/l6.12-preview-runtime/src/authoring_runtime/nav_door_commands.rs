// Nav/Door/Inventory Command Handlers
// Deferred: re-integrate against the new engine navigation/door/inventory API when it is available.
// Currently returns stub observations to maintain vertical slice end-to-end flow.

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::NavDoorInventoryCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: NavDoorInventoryCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        NavDoorInventoryCommand::OpenDoor => {
            session.proof_state.open_door();
            Ok(EditorAuthoringObservation::DoorOpened)
        }
        NavDoorInventoryCommand::CloseDoor => {
            session.proof_state.close_door();
            Ok(EditorAuthoringObservation::DoorClosed)
        }
        NavDoorInventoryCommand::SetDoorBlocked { reason } => {
            session.proof_state.block_door(reason.clone());
            Ok(EditorAuthoringObservation::DoorBlocked { reason })
        }
        NavDoorInventoryCommand::SetDoorLocked => {
            session.proof_state.lock_door();
            Ok(EditorAuthoringObservation::DoorLocked)
        }
        NavDoorInventoryCommand::GetDoorState => {
            let state = session.proof_state.door_inventory.door_state.as_str().to_string();
            let blocked_reason = session.proof_state.door_inventory.blocked_reason.clone();
            Ok(EditorAuthoringObservation::DoorStateInfo {
                state,
                blocked_reason,
            })
        }
        NavDoorInventoryCommand::SetNavigationPath { start, destination } => {
            session.proof_state.door_inventory.navigation_path = Some((start, destination));
            Ok(EditorAuthoringObservation::NavigationPathSet { start, destination })
        }
        NavDoorInventoryCommand::GetNavigationStatus => {
            let (status, blocked_reason) = session.proof_state.navigation_status();
            Ok(EditorAuthoringObservation::NavigationStatusInfo {
                status,
                blocked_reason,
            })
        }
        NavDoorInventoryCommand::AddItemToInventory {
            item_id, item_name, ..
        } => Ok(EditorAuthoringObservation::ItemAddedToInventory { item_id, item_name }),
        NavDoorInventoryCommand::RemoveItemFromInventory { item_id } => {
            Ok(EditorAuthoringObservation::ItemRemovedFromInventory { item_id })
        }
        NavDoorInventoryCommand::TransferItemContainerToInventory { item_id } => {
            Ok(EditorAuthoringObservation::ItemTransferredToInventory { item_id })
        }
        NavDoorInventoryCommand::TransferItemInventoryToContainer { item_id } => {
            Ok(EditorAuthoringObservation::ItemTransferredToContainer { item_id })
        }
        NavDoorInventoryCommand::EquipWeapon { item_id } => {
            Ok(EditorAuthoringObservation::WeaponEquipped { item_id })
        }
        NavDoorInventoryCommand::UnequipWeapon => Ok(EditorAuthoringObservation::WeaponUnequipped),
        NavDoorInventoryCommand::GetInventoryState => {
            Ok(EditorAuthoringObservation::InventoryStateInfo {
                items: vec![],
                equipped_weapon: None,
            })
        }
        NavDoorInventoryCommand::GetContainerState => {
            Ok(EditorAuthoringObservation::ContainerStateInfo { items: vec![] })
        }
        NavDoorInventoryCommand::SaveProofSceneState => {
            Ok(EditorAuthoringObservation::ProofSceneStateSaved {
                state_json: String::new(),
            })
        }
        NavDoorInventoryCommand::LoadProofSceneState { .. } => {
            Ok(EditorAuthoringObservation::ProofSceneStateLoaded)
        }
        NavDoorInventoryCommand::ResetProofSceneBaseline => {
            Ok(EditorAuthoringObservation::ProofSceneBaselineReset)
        }
        NavDoorInventoryCommand::SaveFullWorldState => {
            Ok(EditorAuthoringObservation::FullWorldStateSaved {
                state_json: String::new(),
                world_version: 0,
                save_timestamp: 0.0,
            })
        }
        NavDoorInventoryCommand::LoadFullWorldState { .. } => {
            Ok(EditorAuthoringObservation::FullWorldStateLoaded { world_version: 0 })
        }
        NavDoorInventoryCommand::GetWorldStateMetadata => {
            Ok(EditorAuthoringObservation::WorldStateMetadata {
                world_version: 0,
                simulation_time: 0.0,
                domains_count: 0,
            })
        }
        NavDoorInventoryCommand::RequestRegionLoad { region_key } => {
            Ok(EditorAuthoringObservation::RegionLoadRequested { region_key })
        }
        NavDoorInventoryCommand::CompleteRegionLoad {
            region_key,
            size_bytes,
        } => Ok(EditorAuthoringObservation::RegionLoadCompleted {
            region_key,
            size_bytes,
        }),
        NavDoorInventoryCommand::RequestRegionUnload { region_key } => {
            Ok(EditorAuthoringObservation::RegionUnloadRequested { region_key })
        }
        NavDoorInventoryCommand::CompleteRegionUnload { region_key } => {
            Ok(EditorAuthoringObservation::RegionUnloadCompleted { region_key })
        }
        NavDoorInventoryCommand::GetMemoryPressure => {
            Ok(EditorAuthoringObservation::MemoryPressure {
                pressure: "low".to_string(),
                current_bytes: 0,
                budget_bytes: 1073741824,
            })
        }
        NavDoorInventoryCommand::GetRegionResidency { region_key } => {
            Ok(EditorAuthoringObservation::RegionResidency {
                region_key,
                residency_state: "resident".to_string(),
            })
        }
        NavDoorInventoryCommand::GetResidentRegions => {
            Ok(EditorAuthoringObservation::ResidentRegions {
                regions: vec![],
                count: 0,
            })
        }
        NavDoorInventoryCommand::GetMemoryUsage => Ok(EditorAuthoringObservation::MemoryUsage {
            current_bytes: 0,
            budget_bytes: 1073741824,
            usage_percent: 0.0,
        }),
        NavDoorInventoryCommand::WorldPosToRegion { position } => {
            let region_key = (0, 0, 0);
            Ok(EditorAuthoringObservation::RegionKeyFromPosition {
                position,
                region_key,
            })
        }
    }
}
