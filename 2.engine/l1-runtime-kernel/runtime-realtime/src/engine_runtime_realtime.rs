use engine_runtime::{
    EngineCoreError, EngineCoreResult, ExecutionResult, PresentableFrame, RuntimeConfig,
    RuntimeKernel, RuntimeProfile,
};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeRuntimeConfig {
    pub target_fps: u16,
    pub visibility_freshness_frames: u8,
    pub enqueue_presentable_frames: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeExecutionResult {
    pub kernel_result: ExecutionResult,
    pub frame_presented: bool,
}

#[derive(Debug)]
pub struct RealtimeRuntimeProfile {
    kernel: RuntimeKernel,
    config: RealtimeRuntimeConfig,
    next_frame_id: u64,
}

impl RealtimeRuntimeProfile {
    pub fn new(world: WorldState, config: RealtimeRuntimeConfig) -> EngineCoreResult<Self> {
        if config.target_fps == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "realtime target_fps must be non-zero",
            ));
        }
        let kernel = RuntimeKernel::new(
            world,
            RuntimeConfig {
                profile: RuntimeProfile::Interactive60,
                max_apply_segments_per_tick: engine_world::MAX_SEGMENTS_PER_TICK,
                publish_passes: 1,
            },
        );
        Ok(Self {
            kernel,
            config,
            next_frame_id: 1,
        })
    }
    pub fn kernel(&self) -> &RuntimeKernel {
        &self.kernel
    }
    pub fn kernel_mut(&mut self) -> &mut RuntimeKernel {
        &mut self.kernel
    }
    pub fn step(&mut self) -> EngineCoreResult<RealtimeExecutionResult> {
        if self.config.enqueue_presentable_frames {
            self.kernel.enqueue_presentable_frame(PresentableFrame {
                frame_id: self.next_frame_id,
                visibility_freshness_frames: self.config.visibility_freshness_frames,
            })?;
            self.next_frame_id = self.next_frame_id.saturating_add(1);
        }
        let kernel_result = self.kernel.run_tick()?;
        Ok(RealtimeExecutionResult {
            frame_presented: kernel_result.presented_frame,
            kernel_result,
        })
    }
}
