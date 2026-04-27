use engine_runtime_headless::*;
use engine_runtime::ExecutionResult;
use engine_core::Tick;
use engine_world::WorldState;

#[test]
fn test_headless_config_creation() {
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 5,
        emit_snapshot_bytes: true,
    };
    assert_eq!(config.snapshot_segment_count, 5);
    assert!(config.emit_snapshot_bytes);
}

#[test]
fn test_headless_config_no_snapshots() {
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 0,
        emit_snapshot_bytes: false,
    };
    assert!(!config.emit_snapshot_bytes);
}

#[test]
fn test_headless_config_equality() {
    let a = HeadlessRuntimeConfig {
        snapshot_segment_count: 3,
        emit_snapshot_bytes: true,
    };
    let b = HeadlessRuntimeConfig {
        snapshot_segment_count: 3,
        emit_snapshot_bytes: true,
    };
    let c = HeadlessRuntimeConfig {
        snapshot_segment_count: 5,
        emit_snapshot_bytes: true,
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_execution_result_with_snapshot() {
    let kernel_result = ExecutionResult {
        tick: Tick(1),
        applied_segments: 2,
        published_records: 0,
        presented_frame: false,
    };
    let result = HeadlessExecutionResult {
        kernel_result: kernel_result.clone(),
        snapshot_bytes: Some(vec![1, 2, 3]),
    };
    assert!(result.snapshot_bytes.is_some());
    assert_eq!(result.kernel_result.tick, Tick(1));
}

#[test]
fn test_execution_result_without_snapshot() {
    let kernel_result = ExecutionResult {
        tick: Tick(1),
        applied_segments: 0,
        published_records: 0,
        presented_frame: false,
    };
    let result = HeadlessExecutionResult {
        kernel_result,
        snapshot_bytes: None,
    };
    assert!(result.snapshot_bytes.is_none());
}

#[test]
fn test_headless_runtime_profile_creation() {
    let world = WorldState::new();
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 3,
        emit_snapshot_bytes: false,
    };
    let profile = HeadlessRuntimeProfile::new(world, config);
    assert_eq!(profile.kernel().world().current_tick(), Tick(0));
}

#[test]
fn test_headless_runtime_profile_step_without_snapshots() {
    let world = WorldState::new();
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 1,
        emit_snapshot_bytes: false,
    };
    let mut profile = HeadlessRuntimeProfile::new(world, config);
    let result = profile.step().unwrap();
    assert!(result.snapshot_bytes.is_none());
    assert_eq!(result.kernel_result.tick, Tick(1));
}

#[test]
fn test_headless_runtime_profile_step_with_snapshots() {
    let world = WorldState::new();
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 3,
        emit_snapshot_bytes: true,
    };
    let mut profile = HeadlessRuntimeProfile::new(world, config);
    let result = profile.step().unwrap();
    assert!(result.snapshot_bytes.is_some());
    assert!(!result.snapshot_bytes.unwrap().is_empty());
}

#[test]
fn test_headless_runtime_profile_multiple_steps() {
    let world = WorldState::new();
    let config = HeadlessRuntimeConfig {
        snapshot_segment_count: 1,
        emit_snapshot_bytes: false,
    };
    let mut profile = HeadlessRuntimeProfile::new(world, config);
    assert_eq!(profile.step().unwrap().kernel_result.tick, Tick(1));
    assert_eq!(profile.step().unwrap().kernel_result.tick, Tick(2));
    assert_eq!(profile.step().unwrap().kernel_result.tick, Tick(3));
}
