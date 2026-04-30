use engine_runtime::{
    EngineCoreError, EngineCoreResult, ExecutionResult, PresentableFrame, RuntimeConfig,
    RuntimeKernel, RuntimeProfile,
};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub const REALTIME_TARGET_FPS: u16 = 60;

/// Unique identifier for a realtime frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RealtimeFrameId(pub u64);

/// Cadence control for realtime frame execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeFrameCadence {
    pub target_fps: u16,
    pub frame_budget_micros: u32,
}

impl RealtimeFrameCadence {
    pub fn from_fps(fps: u16) -> EngineCoreResult<Self> {
        if fps == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "realtime target_fps must be non-zero",
            ));
        }
        Ok(Self {
            target_fps: fps,
            frame_budget_micros: 1_000_000 / u32::from(fps),
        })
    }
}

/// Receipt confirming a realtime frame execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeFrameReceipt {
    pub frame_id: RealtimeFrameId,
    pub presented: bool,
    pub tick: engine_core::Tick,
    pub frame_budget_micros: u32,
}

/// Decision on presentation budget allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresentationBudgetDecision {
    Present,
    Skip,
    Degrade,
}

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
    pub frame_id: RealtimeFrameId,
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
        let frame_id = RealtimeFrameId(self.next_frame_id);
        self.next_frame_id = self.next_frame_id.saturating_add(1);
        if self.config.enqueue_presentable_frames {
            self.kernel.enqueue_presentable_frame(PresentableFrame {
                frame_id: frame_id.0,
                visibility_freshness_frames: self.config.visibility_freshness_frames,
            })?;
        }
        let kernel_result = self.kernel.run_tick()?;
        Ok(RealtimeExecutionResult {
            frame_presented: kernel_result.presented_frame,
            frame_id,
            kernel_result,
        })
    }

    /// Execute frame with receipt.
    pub fn step_with_receipt(&mut self) -> EngineCoreResult<RealtimeFrameReceipt> {
        let cadence = self.cadence()?;
        let result = self.step()?;
        Ok(RealtimeFrameReceipt {
            frame_id: result.frame_id,
            presented: result.frame_presented,
            tick: result.kernel_result.tick,
            frame_budget_micros: cadence.frame_budget_micros,
        })
    }

    /// Get frame cadence.
    pub fn cadence(&self) -> EngineCoreResult<RealtimeFrameCadence> {
        RealtimeFrameCadence::from_fps(self.config.target_fps)
    }

    /// Make presentation budget decision.
    pub fn presentation_budget_decision(
        &self,
        frame_time_micros: u32,
    ) -> EngineCoreResult<PresentationBudgetDecision> {
        let cadence = self.cadence()?;
        Ok(if frame_time_micros <= cadence.frame_budget_micros {
            PresentationBudgetDecision::Present
        } else if frame_time_micros <= cadence.frame_budget_micros.saturating_mul(2) {
            PresentationBudgetDecision::Degrade
        } else {
            PresentationBudgetDecision::Skip
        })
    }
}
