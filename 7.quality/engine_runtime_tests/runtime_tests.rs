use engine_runtime::*;
use engine_core::Tick;
use engine_world::WorldState;

// === Tick Tests ===

#[test]
fn test_tick_creation() {
    let tick = Tick(42);
    assert_eq!(tick, Tick(42));
}

#[test]
fn test_tick_ordering() {
    assert!(Tick(0) < Tick(1));
    assert!(Tick(100) > Tick(99));
}

#[test]
fn test_tick_equality() {
    assert_eq!(Tick(5), Tick(5));
    assert_ne!(Tick(5), Tick(6));
}

// === RuntimeProfile Tests ===

#[test]
fn test_runtime_profile_variants() {
    let headless = RuntimeProfile::Headless20;
    let interactive = RuntimeProfile::Interactive60;
    assert_ne!(headless, interactive);
}

#[test]
fn test_runtime_profile_equality() {
    assert_eq!(RuntimeProfile::Headless20, RuntimeProfile::Headless20);
    assert_eq!(RuntimeProfile::Interactive60, RuntimeProfile::Interactive60);
}

// === RuntimeConfig Tests ===

#[test]
fn test_runtime_config_creation() {
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 256,
        publish_passes: 1,
    };
    assert_eq!(config.profile, RuntimeProfile::Interactive60);
    assert_eq!(config.max_apply_segments_per_tick, 256);
}

#[test]
fn test_runtime_config_equality() {
    let a = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 128,
        publish_passes: 2,
    };
    let b = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 128,
        publish_passes: 2,
    };
    let c = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 128,
        publish_passes: 2,
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// === PresentableFrame Tests ===

#[test]
fn test_presentable_frame_creation() {
    let frame = PresentableFrame {
        frame_id: 1,
        visibility_freshness_frames: 3,
    };
    assert_eq!(frame.frame_id, 1);
    assert_eq!(frame.visibility_freshness_frames, 3);
}

#[test]
fn test_presentable_frame_equality() {
    let a = PresentableFrame {
        frame_id: 5,
        visibility_freshness_frames: 2,
    };
    let b = PresentableFrame {
        frame_id: 5,
        visibility_freshness_frames: 2,
    };
    let c = PresentableFrame {
        frame_id: 6,
        visibility_freshness_frames: 2,
    };
    assert_eq!(a, b);
    assert_ne!(a, c);
}

// === ExecutionResult Tests ===

#[test]
fn test_execution_result_creation() {
    let result = ExecutionResult {
        tick: Tick(1),
        applied_segments: 5,
        published_records: 2,
        presented_frame: true,
    };
    assert_eq!(result.tick, Tick(1));
    assert_eq!(result.applied_segments, 5);
    assert!(result.presented_frame);
}

#[test]
fn test_execution_result_equality() {
    let a = ExecutionResult {
        tick: Tick(10),
        applied_segments: 3,
        published_records: 1,
        presented_frame: false,
    };
    let b = ExecutionResult {
        tick: Tick(10),
        applied_segments: 3,
        published_records: 1,
        presented_frame: false,
    };
    assert_eq!(a, b);
}

// === RuntimeKernel Tests ===

#[test]
fn test_runtime_kernel_creation() {
    let world = WorldState::new();
    let config = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 256,
        publish_passes: 1,
    };
    let kernel = RuntimeKernel::new(world, config);
    assert_eq!(kernel.world().current_tick(), Tick(0));
}

#[test]
fn test_runtime_kernel_world_access() {
    let world = WorldState::new();
    let config = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 256,
        publish_passes: 1,
    };
    let kernel = RuntimeKernel::new(world, config);
    assert_eq!(kernel.world().current_tick(), Tick(0));
}

#[test]
fn test_runtime_kernel_run_tick() {
    let world = WorldState::new();
    let config = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 256,
        publish_passes: 1,
    };
    let mut kernel = RuntimeKernel::new(world, config);
    
    let result = kernel.run_tick().unwrap();
    assert_eq!(result.tick, Tick(1));
}

#[test]
fn test_runtime_kernel_multiple_ticks() {
    let world = WorldState::new();
    let config = RuntimeConfig {
        profile: RuntimeProfile::Headless20,
        max_apply_segments_per_tick: 256,
        publish_passes: 1,
    };
    let mut kernel = RuntimeKernel::new(world, config);
    
    kernel.run_tick().unwrap();
    assert_eq!(kernel.world().current_tick(), Tick(1));
    
    kernel.run_tick().unwrap();
    assert_eq!(kernel.world().current_tick(), Tick(2));
}

#[test]
fn test_runtime_kernel_enqueue_frame() {
    let world = WorldState::new();
    let config = RuntimeConfig {
        profile: RuntimeProfile::Interactive60,
        max_apply_segments_per_tick: 256,
        publish_passes: 1,
    };
    let mut kernel = RuntimeKernel::new(world, config);
    
    kernel.enqueue_presentable_frame(PresentableFrame {
        frame_id: 1,
        visibility_freshness_frames: 2,
    }).unwrap();
}

// === EngineCoreError Tests ===

#[test]
fn test_engine_core_error_invalid_descriptor() {
    let error = EngineCoreError::InvalidDescriptor("test error");
    let msg = format!("{}", error);
    assert!(msg.contains("test error"));
}

// === EngineCoreResult Tests ===

#[test]
fn test_engine_core_result_ok() {
    let result: EngineCoreResult<i32> = Ok(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_engine_core_result_err() {
    let result: EngineCoreResult<()> = Err(EngineCoreError::InvalidDescriptor("fail"));
    assert!(result.is_err());
}
