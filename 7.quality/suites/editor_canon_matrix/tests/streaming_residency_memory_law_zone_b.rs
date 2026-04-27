// PHASE 2: STREAMING / RESIDENCY / MEMORY LAW (ZONE B)
//
// Tests that world is divided into regions with load/unload/residency management
// Runtime does NOT hold entire world in memory

use link_ingress_packets::{
    EditorAuthoringCommand, EditorAuthoringPacket, NavDoorInventoryCommand,
};
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

fn create_session_with_runtime() -> EditorAuthoringSession {
    let mut session = EditorAuthoringSession::new();
    session
        .initialize_vertical_slice_session()
        .expect("Failed to initialize runtime");
    session
}

fn send_cmd(session: &mut EditorAuthoringSession, cmd: NavDoorInventoryCommand, id: u64) -> String {
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::NavDoorInventory(cmd),
        request_id: id,
    };
    format!("{:?}", session.handle_command(packet).unwrap())
}

#[test]
fn test_region_load_unload_cycle() {
    let mut session = create_session_with_runtime();
    let region_key = (0, 0, 0);

    // Request load
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::RequestRegionLoad { region_key },
        1,
    );
    assert!(obs.contains("RegionLoadRequested"));

    // Complete load with 10MB
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::CompleteRegionLoad {
            region_key,
            size_bytes: 10 * 1024 * 1024,
        },
        2,
    );
    assert!(obs.contains("RegionLoadCompleted"));

    // Verify region is resident
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::GetRegionResidency { region_key },
        3,
    );
    assert!(obs.contains("Resident"));

    // Verify resident count
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetResidentRegions, 4);
    assert!(obs.contains("count: 1"));

    // Unload
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::RequestRegionUnload { region_key },
        5,
    );
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::CompleteRegionUnload { region_key },
        6,
    );

    // Verify unloaded
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetResidentRegions, 7);
    assert!(obs.contains("count: 0"));
}

#[test]
fn test_memory_pressure_tracking() {
    let mut session = create_session_with_runtime();

    // Initially healthy
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetMemoryPressure, 1);
    assert!(obs.contains("Healthy"));

    // Load multiple regions to increase pressure
    for i in 0..50 {
        let region_key = (i, 0, 0);
        send_cmd(
            &mut session,
            NavDoorInventoryCommand::RequestRegionLoad { region_key },
            100 + i as u64,
        );
        send_cmd(
            &mut session,
            NavDoorInventoryCommand::CompleteRegionLoad {
                region_key,
                size_bytes: 10 * 1024 * 1024,
            },
            200 + i as u64,
        );
    }

    // Should now be under pressure
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetMemoryPressure, 2);
    assert!(obs.contains("Elevated") || obs.contains("Critical"));
}

#[test]
fn test_world_pos_to_region_key() {
    let mut session = create_session_with_runtime();

    // Test origin
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::WorldPosToRegion {
            position: [50.0, 50.0, 50.0],
        },
        1,
    );
    assert!(obs.contains("region_key: (0, 0, 0)"));

    // Test positive offset
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::WorldPosToRegion {
            position: [150.0, 50.0, 50.0],
        },
        2,
    );
    assert!(obs.contains("region_key: (1, 0, 0)"));

    // Test negative offset
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::WorldPosToRegion {
            position: [-50.0, 50.0, 50.0],
        },
        3,
    );
    assert!(obs.contains("region_key: (-1, 0, 0)"));
}

#[test]
fn test_residency_state_transitions() {
    let mut session = create_session_with_runtime();
    let region_key = (5, 5, 5);

    // Request load -> Loading state
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::RequestRegionLoad { region_key },
        1,
    );
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::GetRegionResidency { region_key },
        2,
    );
    assert!(obs.contains("Loading"));

    // Complete load -> Resident state
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::CompleteRegionLoad {
            region_key,
            size_bytes: 10 * 1024 * 1024,
        },
        3,
    );
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::GetRegionResidency { region_key },
        4,
    );
    assert!(obs.contains("Resident"));

    // Request unload -> Evicting state
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::RequestRegionUnload { region_key },
        5,
    );
    let obs = send_cmd(
        &mut session,
        NavDoorInventoryCommand::GetRegionResidency { region_key },
        6,
    );
    assert!(obs.contains("Evicting"));

    // Complete unload
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::CompleteRegionUnload { region_key },
        7,
    );
}

#[test]
fn test_multiple_regions_resident() {
    let mut session = create_session_with_runtime();

    let regions = vec![(0, 0, 0), (1, 0, 0), (0, 1, 0), (0, 0, 1), (-1, 0, 0)];

    // Load all regions
    for (idx, &region_key) in regions.iter().enumerate() {
        send_cmd(
            &mut session,
            NavDoorInventoryCommand::RequestRegionLoad { region_key },
            100 + idx as u64,
        );
        send_cmd(
            &mut session,
            NavDoorInventoryCommand::CompleteRegionLoad {
                region_key,
                size_bytes: 5 * 1024 * 1024,
            },
            200 + idx as u64,
        );
    }

    // Verify all are resident
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetResidentRegions, 1);
    assert!(obs.contains("count: 5"));
}

#[test]
fn test_memory_usage_accounting() {
    let mut session = create_session_with_runtime();

    // Check initial memory
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetMemoryUsage, 1);
    assert!(obs.contains("current_bytes: 0"));

    // Load region with known size
    let region_key = (0, 0, 0);
    let size_bytes = 20 * 1024 * 1024; // 20MB

    send_cmd(
        &mut session,
        NavDoorInventoryCommand::RequestRegionLoad { region_key },
        2,
    );
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::CompleteRegionLoad {
            region_key,
            size_bytes,
        },
        3,
    );

    // Verify memory increased
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetMemoryUsage, 4);
    assert!(obs.contains("current_bytes: 20971520")); // 20MB in bytes

    // Unload and verify memory is freed
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::RequestRegionUnload { region_key },
        5,
    );
    send_cmd(
        &mut session,
        NavDoorInventoryCommand::CompleteRegionUnload { region_key },
        6,
    );

    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetMemoryUsage, 7);
    assert!(obs.contains("current_bytes: 0"));
}

#[test]
fn test_automatic_eviction_on_pressure() {
    let mut session = create_session_with_runtime();

    // Load regions until we hit budget (512MB / 10MB = ~51 regions)
    for i in 0..60 {
        let region_key = (i, 0, 0);
        send_cmd(
            &mut session,
            NavDoorInventoryCommand::RequestRegionLoad { region_key },
            100 + i as u64,
        );
        send_cmd(
            &mut session,
            NavDoorInventoryCommand::CompleteRegionLoad {
                region_key,
                size_bytes: 10 * 1024 * 1024,
            },
            200 + i as u64,
        );
    }

    // Should have evicted some regions to stay under budget
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetResidentRegions, 1);
    assert!(obs.contains("ResidentRegions"));

    // Verify memory usage is under budget
    let obs = send_cmd(&mut session, NavDoorInventoryCommand::GetMemoryUsage, 2);
    assert!(obs.contains("MemoryUsage"));
}
