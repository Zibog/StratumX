use engine_runtime::{
    EngineCoreResult, ExecutionResult, RuntimeConfig, RuntimeKernel, RuntimeProfile,
};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadlessRuntimeConfig {
    pub snapshot_segment_count: usize,
    pub emit_snapshot_bytes: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadlessExecutionResult {
    pub kernel_result: ExecutionResult,
    pub snapshot_bytes: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct HeadlessRuntimeProfile {
    kernel: RuntimeKernel,
    config: HeadlessRuntimeConfig,
}

impl HeadlessRuntimeProfile {
    pub fn new(world: WorldState, config: HeadlessRuntimeConfig) -> Self {
        let kernel = RuntimeKernel::new(
            world,
            RuntimeConfig {
                profile: RuntimeProfile::Headless20,
                max_apply_segments_per_tick: engine_world::MAX_SEGMENTS_PER_TICK,
                publish_passes: 1,
            },
        );
        Self { kernel, config }
    }
    pub fn kernel(&self) -> &RuntimeKernel {
        &self.kernel
    }
    pub fn kernel_mut(&mut self) -> &mut RuntimeKernel {
        &mut self.kernel
    }
    pub fn step(&mut self) -> EngineCoreResult<HeadlessExecutionResult> {
        let kernel_result = self.kernel.run_tick()?;
        let snapshot_bytes = if self.config.emit_snapshot_bytes {
            Some(
                self.kernel
                    .world()
                    .snapshot_bytes(self.config.snapshot_segment_count)?,
            )
        } else {
            None
        };
        Ok(HeadlessExecutionResult {
            kernel_result,
            snapshot_bytes,
        })
    }
}
