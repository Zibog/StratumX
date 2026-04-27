use engine_runtime_realtime::*;
use engine_runtime::ExecutionResult;
use engine_core::Tick;
use engine_world::WorldState;

// === RealtimeRuntimeConfig Tests ===

#[test]
fn test_realtime_config_creation() {
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 3,
        enqueue_presentable_frames: true,
    };
    assert_eq!(config.target_fps, 60);
    assert!(config.enqueue_presentable_frames);
}

#[test]
fn test_realtime_config_no_frames() {
    let config = RealtimeRuntimeConfig {
        target_fps: 30,
        visibility_freshness_frames: 0,
        enqueue_presentable_frames: false,
    };
    assert!(!config.enqueue_presentable_frames);
}

#[test]
fn test_realtime_config_equality() {
    let a = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: true,
    };
    let b = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: true,
    };
    let c = RealtimeRuntimeConfig {
        target_fps: 30,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: true,
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// === RealtimeExecutionResult Tests ===

#[test]
fn test_execution_result_frame_presented() {
    let kernel_result = ExecutionResult {
        tick: Tick(1),
        applied_segments: 1,
        published_records: 0,
        presented_frame: true,
    };
    let result = RealtimeExecutionResult {
        kernel_result: kernel_result.clone(),
        frame_presented: true,
    };
    assert!(result.frame_presented);
    assert_eq!(result.kernel_result.tick, Tick(1));
}

#[test]
fn test_execution_result_no_frame() {
    let kernel_result = ExecutionResult {
        tick: Tick(1),
        applied_segments: 0,
        published_records: 0,
        presented_frame: false,
    };
    let result = RealtimeExecutionResult {
        kernel_result,
        frame_presented: false,
    };
    assert!(!result.frame_presented);
}

// === RealtimeRuntimeProfile Tests ===

#[test]
fn test_realtime_runtime_profile_creation() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: false,
    };
    let profile = RealtimeRuntimeProfile::new(world, config);
    assert!(profile.is_ok());
}

#[test]
fn test_realtime_runtime_profile_rejects_zero_fps() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 0,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: false,
    };
    let profile = RealtimeRuntimeProfile::new(world, config);
    assert!(profile.is_err());
}

#[test]
fn test_realtime_runtime_profile_kernel_access() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: false,
    };
    let profile = RealtimeRuntimeProfile::new(world, config).unwrap();
    assert_eq!(profile.kernel().world().current_tick(), Tick(0));
}

#[test]
fn test_realtime_runtime_profile_kernel_mut() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: false,
    };
    let mut profile = RealtimeRuntimeProfile::new(world, config).unwrap();
    let _kernel_mut = profile.kernel_mut();
}

#[test]
fn test_realtime_runtime_profile_step_without_frames() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: false,
    };
    let mut profile = RealtimeRuntimeProfile::new(world, config).unwrap();
    let result = profile.step().unwrap();
    assert_eq!(result.kernel_result.tick, Tick(1));
}

#[test]
fn test_realtime_runtime_profile_step_with_frames() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 2,
        enqueue_presentable_frames: true,
    };
    let mut profile = RealtimeRuntimeProfile::new(world, config).unwrap();
    let result = profile.step().unwrap();
    assert_eq!(result.kernel_result.tick, Tick(1));
}

#[test]
fn test_realtime_runtime_profile_multiple_steps() {
    let world = WorldState::new();
    let config = RealtimeRuntimeConfig {
        target_fps: 60,
        visibility_freshness_frames: 1,
        enqueue_presentable_frames: false,
    };
    let mut profile = RealtimeRuntimeProfile::new(world, config).unwrap();
    assert_eq!(profile.step().unwrap().kernel_result.tick, Tick(1));
    assert_eq!(profile.step().unwrap().kernel_result.tick, Tick(2));
    assert_eq!(profile.step().unwrap().kernel_result.tick, Tick(3));
}

// === Serialization Tests ===

#[test]
fn test_realtime_config_serialization_roundtrip() {
    let config = RealtimeRuntimeConfig {
        target_fps: 120,
        visibility_freshness_frames: 5,
        enqueue_presentable_frames: true,
    };
    let json = serde_json::to_string(&config).unwrap();
    let loaded: RealtimeRuntimeConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config, loaded);
}

#[test]
fn test_execution_result_serialization_roundtrip() {
    let kernel_result = ExecutionResult {
        tick: Tick(42),
        applied_segments: 3,
        published_records: 1,
        presented_frame: true,
    };
    let result = RealtimeExecutionResult {
        kernel_result,
        frame_presented: true,
    };
    let json = serde_json::to_string(&result).unwrap();
    let loaded: RealtimeExecutionResult = serde_json::from_str(&json).unwrap();
    assert_eq!(result.kernel_result.tick, loaded.kernel_result.tick);
    assert_eq!(result.frame_presented, loaded.frame_presented);
}
