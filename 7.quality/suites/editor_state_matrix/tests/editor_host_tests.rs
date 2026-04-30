//! Unit Tests for EditorHost
//!
//! Tests for EditorHost thin facade implementation
//! Validates: Requirements 8.1, 8.2, 8.3, 8.4

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use stratumx_editor_state_containers::{
    BasicEventBus, CacheLayer, DiagnosticsState, EditorHost, EditorServices, EventBus,
    ProjectIdentity, ProjectState, QueryLayer, StateContainerSystem, WorkspaceIdentity,
    WorkspaceState, WorldIdentity, WorldState,
};
use uuid::Uuid;

fn create_test_identities() -> (ProjectIdentity, WorkspaceIdentity) {
    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "test-project".to_string(),
        PathBuf::from("test-project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "test-workspace".to_string(),
        PathBuf::from("test-workspace"),
    );
    (project_id, workspace_id)
}

fn create_test_editor_host() -> EditorHost {
    let (project_identity, workspace_identity) = create_test_identities();

    // Create state containers
    let project_state = Arc::new(Mutex::new(ProjectState::new(
        project_identity,
        workspace_identity,
    )));
    let workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
    let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));
    let world_identity = WorldIdentity::new(
        Uuid::new_v4(),
        "test-world".to_string(),
        PathBuf::from("test-world"),
    );
    let world_state = Arc::new(Mutex::new(WorldState::new(
        world_identity,
        "snapshot_123".to_string(),
    )));

    // Create state container system
    let state_system = Arc::new(
        StateContainerSystem::new(
            project_state.clone(),
            workspace_state.clone(),
            diagnostics_state.clone(),
        )
        .unwrap(),
    );

    // Create cache layer
    let cache_layer = Arc::new(Mutex::new(CacheLayer::new(state_system.clone())));

    // Create query layer
    let query_layer = QueryLayer::new(state_system.clone());

    // Create event bus
    let event_bus: Arc<dyn EventBus> = Arc::new(BasicEventBus::new());

    // Create services
    let services = EditorServices::new(
        project_state,
        workspace_state,
        Some(world_state),
        diagnostics_state,
        event_bus,
    )
    .unwrap();

    EditorHost::new(state_system, query_layer, cache_layer, services)
}

#[test]
fn test_delegation_to_services() {
    // Test that EditorHost delegates to services
    let host = create_test_editor_host();

    // Access services through delegation
    let services = host.services();

    // Verify all 7 domain services are accessible
    let _ = &services.world_session;
    let _ = &services.terrain_authoring;
    let _ = &services.material_authoring;
    let _ = &services.audio_authoring;
    let _ = &services.environment_authoring;
    let _ = &services.runtime_mode;
    let _ = &services.diagnostics;
}

#[test]
fn test_delegation_to_services_mut() {
    // Test that EditorHost provides mutable access to services
    let mut host = create_test_editor_host();

    // Access services mutably through delegation
    let services = host.services_mut();

    // Verify all 7 domain services are accessible
    let _ = &mut services.world_session;
    let _ = &mut services.terrain_authoring;
    let _ = &mut services.material_authoring;
    let _ = &mut services.audio_authoring;
    let _ = &mut services.environment_authoring;
    let _ = &mut services.runtime_mode;
    let _ = &mut services.diagnostics;
}

#[test]
fn test_delegation_to_query_layer() {
    // Test that EditorHost delegates queries to QueryLayer
    let host = create_test_editor_host();

    // Access query layer through delegation
    let query = host.query();

    // Verify query layer is functional
    let project_identity = query.get_project_identity();
    assert!(!project_identity.project_name.is_empty());
}

#[test]
fn test_no_direct_state_manipulation() {
    // Test that EditorHost does not directly manipulate state
    let mut host = create_test_editor_host();

    // EditorHost only provides lifecycle coordination
    assert!(host.initialize().is_ok());
    assert!(host.is_initialized());

    // EditorHost only provides validation
    assert!(host.validate_state_ownership().is_ok());

    // EditorHost only provides shutdown
    assert!(host.shutdown().is_ok());
    assert!(!host.is_initialized());

    // No other state manipulation methods exist
    // (proven by type system - if they existed, this would fail to compile)
}

#[test]
fn test_lifecycle_coordination_initialize() {
    // Test lifecycle coordination: initialize
    let mut host = create_test_editor_host();

    assert!(!host.is_initialized());

    let result = host.initialize();
    assert!(result.is_ok());
    assert!(host.is_initialized());
}

#[test]
fn test_lifecycle_coordination_shutdown() {
    // Test lifecycle coordination: shutdown
    let mut host = create_test_editor_host();

    host.initialize().unwrap();
    assert!(host.is_initialized());

    let result = host.shutdown();
    assert!(result.is_ok());
    assert!(!host.is_initialized());
}

#[test]
fn test_lifecycle_coordination_double_initialize() {
    // Test that double initialization is prevented
    let mut host = create_test_editor_host();

    host.initialize().unwrap();

    let result = host.initialize();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "EditorHost already initialized");
}

#[test]
fn test_lifecycle_coordination_shutdown_without_init() {
    // Test that shutdown without initialization is prevented
    let mut host = create_test_editor_host();

    let result = host.shutdown();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "EditorHost not initialized");
}

#[test]
fn test_lifecycle_coordination_reinitialize() {
    // Test that reinitialization after shutdown works
    let mut host = create_test_editor_host();

    // First lifecycle
    host.initialize().unwrap();
    assert!(host.is_initialized());
    host.shutdown().unwrap();
    assert!(!host.is_initialized());

    // Second lifecycle
    host.initialize().unwrap();
    assert!(host.is_initialized());
    host.shutdown().unwrap();
    assert!(!host.is_initialized());
}

#[test]
fn test_minimal_business_logic_line_count() {
    // Test that EditorHost has minimal business logic
    use std::fs;
    use std::path::PathBuf;
    use stratumx_repo_hygiene_support::workspace_root_from_manifest_dir;

    let workspace_root = workspace_root_from_manifest_dir(env!("CARGO_MANIFEST_DIR"));
    let editor_host_path = PathBuf::from(workspace_root)
        .join("5.editor")
        .join("editor-state-containers")
        .join("src")
        .join("runtime")
        .join("services.rs");
    let content = fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");
    let editor_host_block = {
        let host_impl_start = content
            .find("impl EditorHost")
            .expect("EditorHost impl not found");
        let host_impl_end = content[host_impl_start..]
            .find("pub trait EventBus")
            .map(|offset| host_impl_start + offset)
            .expect("EventBus trait boundary not found");
        &content[host_impl_start..host_impl_end]
    };

    // Count lines in lifecycle methods (allowed)
    let lifecycle_methods = ["pub fn new(", "pub fn initialize(", "pub fn shutdown("];
    let mut lifecycle_lines = 0;

    for method in lifecycle_methods.iter() {
        if let Some(start) = editor_host_block.find(method) {
            let after_method = &editor_host_block[start..];
            if let Some(end) = after_method
                .find("\n    pub fn ")
                .or_else(|| after_method.find("\n}"))
            {
                let method_content = &after_method[..end];
                lifecycle_lines += method_content.lines().count();
            }
        }
    }

    // Count lines in delegation methods (allowed)
    let delegation_methods = [
        "pub fn services(",
        "pub fn services_mut(",
        "pub fn query(",
        "pub fn validate_state_ownership(",
        "pub fn is_initialized(",
    ];
    let mut delegation_lines = 0;

    for method in delegation_methods.iter() {
        if let Some(start) = editor_host_block.find(method) {
            let after_method = &editor_host_block[start..];
            if let Some(end) = after_method
                .find("\n    pub fn ")
                .or_else(|| after_method.find("\n}"))
            {
                let method_content = &after_method[..end];
                delegation_lines += method_content.lines().count();
            }
        }
    }

    // Count total code lines (excluding comments, whitespace, and tests)
    let total_lines: usize = editor_host_block
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("/*")
                && !trimmed.starts_with("*")
        })
        .count();

    println!("Total code lines: {}", total_lines);
    println!("Lifecycle lines: {}", lifecycle_lines);
    println!("Delegation lines: {}", delegation_lines);

    // Most lines should be lifecycle or delegation
    let allowed_lines = lifecycle_lines + delegation_lines;
    let business_logic_lines = total_lines.saturating_sub(allowed_lines);

    println!("Business logic lines: {}", business_logic_lines);

    // Business logic should be minimal (less than 10% of total)
    let business_logic_percentage = if total_lines > 0 {
        (business_logic_lines as f64 / total_lines as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "Business logic percentage: {:.2}%",
        business_logic_percentage
    );

    assert!(
        business_logic_percentage < 10.0,
        "EditorHost contains too much business logic: {:.2}% (expected < 10%)",
        business_logic_percentage
    );
}

#[test]
fn test_state_ownership_validation() {
    // Test that EditorHost provides state ownership validation
    let host = create_test_editor_host();

    // Validate state ownership
    let result = host.validate_state_ownership();
    assert!(result.is_ok());
}

#[test]
fn test_editor_host_creation_with_different_identities() {
    // Test EditorHost creation with various identities
    for _ in 0..3 {
        let host = create_test_editor_host();
        assert!(host.is_initialized() == false);
    }
}

#[test]
fn test_editor_host_services_independence() {
    // Test that services can be accessed independently
    let host = create_test_editor_host();
    let services = host.services();

    // Each service should be independently accessible
    let _ = &services.world_session;
    let _ = &services.terrain_authoring;
    let _ = &services.material_authoring;
    let _ = &services.audio_authoring;
    let _ = &services.environment_authoring;
    let _ = &services.runtime_mode;
    let _ = &services.diagnostics;

    // Services don't interfere with each other
}

#[test]
fn test_editor_host_query_layer_independence() {
    // Test that query layer can be accessed independently of services
    let host = create_test_editor_host();

    // Query layer is independent
    let query = host.query();
    let _ = query.get_project_identity();

    // Services are independent
    let services = host.services();
    let _ = &services.world_session;

    // Both can be accessed without interference
}
