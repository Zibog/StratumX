// Nav/Door/Inventory Command Handlers - Module Root

mod door_ops;
mod inventory_ops;
mod nav_ops;
mod state_ops;

use super::session::EditorAuthoringSession;
use link_egress_observations::EditorAuthoringObservation;
use link_ingress_packets::NavDoorInventoryCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: NavDoorInventoryCommand,
) -> Result<EditorAuthoringObservation, String> {
    if let Some(vs) = &mut session.vertical_slice_session {
        match cmd {
            // Door operations
            NavDoorInventoryCommand::OpenDoor => door_ops::handle_open_door(vs),
            NavDoorInventoryCommand::CloseDoor => door_ops::handle_close_door(vs),
            NavDoorInventoryCommand::SetDoorBlocked { reason } => {
                door_ops::handle_set_door_blocked(vs, reason)
            }
            NavDoorInventoryCommand::SetDoorLocked => door_ops::handle_set_door_locked(vs),
            NavDoorInventoryCommand::GetDoorState => door_ops::handle_get_door_state(vs),

            // Navigation operations
            NavDoorInventoryCommand::SetNavigationPath { start, destination } => {
                nav_ops::handle_set_navigation_path(vs, start, destination)
            }
            NavDoorInventoryCommand::GetNavigationStatus => {
                nav_ops::handle_get_navigation_status(vs)
            }

            // Inventory operations
            NavDoorInventoryCommand::AddItemToInventory {
                item_id,
                item_name,
                item_type,
            } => inventory_ops::handle_add_item(vs, item_id, item_name, item_type),
            NavDoorInventoryCommand::RemoveItemFromInventory { item_id } => {
                inventory_ops::handle_remove_item(vs, item_id)
            }
            NavDoorInventoryCommand::TransferItemContainerToInventory { item_id } => {
                inventory_ops::handle_transfer_to_inventory(vs, item_id)
            }
            NavDoorInventoryCommand::TransferItemInventoryToContainer { item_id } => {
                inventory_ops::handle_transfer_to_container(vs, item_id)
            }
            NavDoorInventoryCommand::EquipWeapon { item_id } => {
                inventory_ops::handle_equip_weapon(vs, item_id)
            }
            NavDoorInventoryCommand::UnequipWeapon => inventory_ops::handle_unequip_weapon(vs),
            NavDoorInventoryCommand::GetInventoryState => {
                inventory_ops::handle_get_inventory_state(vs)
            }
            NavDoorInventoryCommand::GetContainerState => {
                inventory_ops::handle_get_container_state(vs)
            }

            // State operations
            NavDoorInventoryCommand::SaveProofSceneState => {
                state_ops::handle_save_proof_scene_state(vs)
            }
            NavDoorInventoryCommand::LoadProofSceneState { state_json } => {
                state_ops::handle_load_proof_scene_state(vs, state_json)
            }
            NavDoorInventoryCommand::ResetProofSceneBaseline => {
                state_ops::handle_reset_proof_scene_baseline(vs)
            }
            NavDoorInventoryCommand::SaveFullWorldState => {
                state_ops::handle_save_full_world_state(vs)
            }
            NavDoorInventoryCommand::LoadFullWorldState { state_json } => {
                state_ops::handle_load_full_world_state(vs, state_json)
            }
            NavDoorInventoryCommand::GetWorldStateMetadata => {
                state_ops::handle_get_world_state_metadata(vs)
            }
        }
    } else {
        Err("Runtime session not initialized".into())
    }
}
