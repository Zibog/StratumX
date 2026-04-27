// ============================================================================
// PACK 5: NAV/DOOR/INVENTORY/PERSISTENCE PROOF-PACK INTEGRATION TESTS
// ============================================================================
//
// These tests prove that editor can runtime-backed author and inspect
// minimal nav/door/inventory/persistence subset through real runtime truth:
// - Navigation path legality depends on door state
// - Door interaction mutates runtime truth
// - Inventory/container/equipment state lives in runtime
// - Save/load/restore preserves runtime state

use link_ingress_packets::*;
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

// ============================================================================
// DOOR INTERACTION TESTS
// ============================================================================

#[test]
fn test_door_state_lives_in_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Get initial door state (should be closed)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::DoorStateInfo { state, .. } =
        result
    {
        assert_eq!(state, "Closed");
    } else {
        panic!("Expected DoorStateInfo");
    }

    // Open door
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::OpenDoor),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::DoorOpened
    ));

    // Verify door is open
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::DoorStateInfo { state, .. } =
        result
    {
        assert_eq!(state, "Open");
    } else {
        panic!("Expected DoorStateInfo");
    }
}

#[test]
fn test_door_state_affects_navigation_legality() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set navigation path through door
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::SetNavigationPath {
                start: [0.0, 0.0, 0.0],
                destination: [0.0, 0.0, 10.0],
            },
        ),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Door is closed, path should be blocked
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetNavigationStatus,
        ),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::NavigationStatusInfo {
        status,
        blocked_reason,
    } = result
    {
        assert_eq!(status, "Blocked");
        assert!(blocked_reason.is_some());
    } else {
        panic!("Expected NavigationStatusInfo");
    }

    // Open door
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::OpenDoor),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Path should now be valid
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetNavigationStatus,
        ),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::NavigationStatusInfo {
        status,
        ..
    } = result
    {
        assert_eq!(status, "Valid");
    } else {
        panic!("Expected NavigationStatusInfo");
    }
}

#[test]
fn test_door_blocked_reason_is_inspectable() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set door blocked with reason
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::SetDoorBlocked {
                reason: "Debris blocking door".to_string(),
            },
        ),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Inspect door state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::DoorStateInfo {
        state,
        blocked_reason,
    } = result
    {
        assert_eq!(state, "Blocked");
        assert_eq!(blocked_reason, Some("Debris blocking door".to_string()));
    } else {
        panic!("Expected DoorStateInfo");
    }

    // Navigation should also show blocked reason
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::SetNavigationPath {
                start: [0.0, 0.0, 0.0],
                destination: [0.0, 0.0, 10.0],
            },
        ),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetNavigationStatus,
        ),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::NavigationStatusInfo {
        status,
        blocked_reason,
    } = result
    {
        assert_eq!(status, "Blocked");
        assert!(blocked_reason.is_some());
    } else {
        panic!("Expected NavigationStatusInfo");
    }
}

// ============================================================================
// INVENTORY/EQUIPMENT TESTS
// ============================================================================

#[test]
fn test_inventory_state_lives_in_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Get initial inventory (should be empty)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        items,
        equipped_weapon,
    } = result
    {
        assert!(items.is_empty());
        assert!(equipped_weapon.is_none());
    } else {
        panic!("Expected InventoryStateInfo");
    }

    // Add item to inventory
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::AddItemToInventory {
                item_id: 100,
                item_name: "Test Weapon".to_string(),
                item_type: "Weapon".to_string(),
            },
        ),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Verify item in inventory
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        items, ..
    } = result
    {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0, 100);
        assert_eq!(items[0].1, "Test Weapon");
    } else {
        panic!("Expected InventoryStateInfo");
    }
}

#[test]
fn test_container_to_inventory_transfer_mutates_runtime() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Get initial container state (should have AK-47)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetContainerState,
        ),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::ContainerStateInfo { items } =
        result
    {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0, 1); // Item ID 1
        assert_eq!(items[0].1, "AK-47");
    } else {
        panic!("Expected ContainerStateInfo");
    }

    // Transfer item from container to inventory
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::TransferItemContainerToInventory { item_id: 1 },
        ),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Verify item removed from container
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetContainerState,
        ),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::ContainerStateInfo { items } =
        result
    {
        assert!(items.is_empty());
    } else {
        panic!("Expected ContainerStateInfo");
    }

    // Verify item added to inventory
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        items, ..
    } = result
    {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].0, 1);
        assert_eq!(items[0].1, "AK-47");
    } else {
        panic!("Expected InventoryStateInfo");
    }
}

#[test]
fn test_equip_unequip_mutates_runtime() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Transfer weapon to inventory
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::TransferItemContainerToInventory { item_id: 1 },
        ),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Equip weapon
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::EquipWeapon {
            item_id: 1,
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Verify weapon equipped
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        equipped_weapon,
        ..
    } = result
    {
        assert_eq!(equipped_weapon, Some(1));
    } else {
        panic!("Expected InventoryStateInfo");
    }

    // Unequip weapon
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::UnequipWeapon),
        request_id: 4,
    };
    session.handle_command(packet).unwrap();

    // Verify weapon unequipped
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 5,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        equipped_weapon,
        ..
    } = result
    {
        assert!(equipped_weapon.is_none());
    } else {
        panic!("Expected InventoryStateInfo");
    }
}

// ============================================================================
// PERSISTENCE TESTS
// ============================================================================

#[test]
fn test_save_load_preserves_runtime_state() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Mutate state: open door, transfer item, equip weapon
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::OpenDoor),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::TransferItemContainerToInventory { item_id: 1 },
        ),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::EquipWeapon {
            item_id: 1,
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Save state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::SaveProofSceneState,
        ),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    let saved_state_json =
        if let link_egress_observations::EditorAuthoringObservation::ProofSceneStateSaved {
            state_json,
        } = result
        {
            state_json
        } else {
            panic!("Expected ProofSceneStateSaved");
        };

    // Mutate state after save
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::CloseDoor),
        request_id: 5,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::UnequipWeapon),
        request_id: 6,
    };
    session.handle_command(packet).unwrap();

    // Load saved state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::LoadProofSceneState {
                state_json: saved_state_json,
            },
        ),
        request_id: 7,
    };
    session.handle_command(packet).unwrap();

    // Verify door is open (restored)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 8,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::DoorStateInfo { state, .. } =
        result
    {
        assert_eq!(state, "Open");
    } else {
        panic!("Expected DoorStateInfo");
    }

    // Verify weapon is equipped (restored)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 9,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        equipped_weapon,
        ..
    } = result
    {
        assert_eq!(equipped_weapon, Some(1));
    } else {
        panic!("Expected InventoryStateInfo");
    }
}

#[test]
fn test_save_load_preserves_destruction_state() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Damage wall
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetWallIntegrity {
            integrity: 0.5,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Set terrain material
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetTerrainMaterial {
            material_type: "Asphalt".to_string(),
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Save state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::SaveProofSceneState,
        ),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    let saved_state_json =
        if let link_egress_observations::EditorAuthoringObservation::ProofSceneStateSaved {
            state_json,
        } = result
        {
            state_json
        } else {
            panic!("Expected ProofSceneStateSaved");
        };

    // Mutate state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::SetWallIntegrity {
            integrity: 1.0,
        }),
        request_id: 4,
    };
    session.handle_command(packet).unwrap();

    // Load saved state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::LoadProofSceneState {
                state_json: saved_state_json,
            },
        ),
        request_id: 5,
    };
    session.handle_command(packet).unwrap();

    // Verify wall integrity restored
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetWallIntegrity),
        request_id: 6,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::WallIntegrityInfo { integrity } =
        result
    {
        assert_eq!(integrity, 0.5);
    } else {
        panic!("Expected WallIntegrityInfo");
    }

    // Verify terrain material restored
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Destruction(DestructionCommand::GetTerrainMaterial),
        request_id: 7,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::TerrainMaterialInfo {
        material_type,
    } = result
    {
        assert_eq!(material_type, "Asphalt");
    } else {
        panic!("Expected TerrainMaterialInfo");
    }
}

#[test]
fn test_reset_baseline_restores_initial_state() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Mutate state
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::OpenDoor),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::TransferItemContainerToInventory { item_id: 1 },
        ),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Reset baseline
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::ResetProofSceneBaseline,
        ),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Verify door is closed (baseline)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::DoorStateInfo { state, .. } =
        result
    {
        assert_eq!(state, "Closed");
    } else {
        panic!("Expected DoorStateInfo");
    }

    // Verify inventory is empty (baseline)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetInventoryState,
        ),
        request_id: 5,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::InventoryStateInfo {
        items, ..
    } = result
    {
        assert!(items.is_empty());
    } else {
        panic!("Expected InventoryStateInfo");
    }

    // Verify container has weapon (baseline)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(
            NavDoorInventoryCommand::GetContainerState,
        ),
        request_id: 6,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::ContainerStateInfo { items } =
        result
    {
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].1, "AK-47");
    } else {
        panic!("Expected ContainerStateInfo");
    }
}

#[test]
fn test_nav_door_inventory_without_runtime_session_fails() {
    let mut session = EditorAuthoringSession::new();
    // Don't initialize runtime session

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(NavDoorInventoryCommand::GetDoorState),
        request_id: 1,
    };
    let result = session.handle_command(packet);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Runtime session not initialized"));
}
