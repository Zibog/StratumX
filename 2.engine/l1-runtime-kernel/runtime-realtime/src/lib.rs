use engine_core::{StableDigest64, StableDigestBuilder};
use engine_runtime::{
    EngineCoreError, ExecutionResult, PresentableFrame, RuntimeConfig, RuntimeKernel,
    RuntimeProfile,
};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub const REALTIME_TARGET_FPS: u16 = 60;

pub type RealtimeRuntimeResult<T> = Result<T, RealtimeRuntimeFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealtimeRuntimeFailureReason {
    InvalidTargetFps,
    InvalidFrameCadence,
    PresentationBudgetExceeded,
    InvalidRuntimeState,
}

impl RealtimeRuntimeFailureReason {
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidTargetFps => "realtime target_fps must be non-zero",
            Self::InvalidFrameCadence => "realtime visibility freshness must be non-zero",
            Self::PresentationBudgetExceeded => "realtime presentation budget exceeded",
            Self::InvalidRuntimeState => "realtime runtime state is invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeRuntimeFailure {
    pub reason: RealtimeRuntimeFailureReason,
    pub digest: StableDigest64,
}

impl RealtimeRuntimeFailure {
    pub fn for_reason(reason: RealtimeRuntimeFailureReason) -> Self {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.runtime.realtime.failure")
            .write_u8(reason as u8);
        Self {
            reason,
            digest: digest.finish(),
        }
    }
}

impl From<RealtimeRuntimeFailure> for EngineCoreError {
    fn from(failure: RealtimeRuntimeFailure) -> Self {
        EngineCoreError::InvalidDescriptor(failure.reason.message())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RealtimeFrameId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeFrameCadence {
    pub target_fps: u16,
    pub frame_budget_micros: u32,
}

impl RealtimeFrameCadence {
    pub fn from_fps(fps: u16) -> RealtimeRuntimeResult<Self> {
        if fps == 0 {
            return Err(RealtimeRuntimeFailure::for_reason(
                RealtimeRuntimeFailureReason::InvalidTargetFps,
            ));
        }
        Ok(Self {
            target_fps: fps,
            frame_budget_micros: 1_000_000 / u32::from(fps),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealtimeFrameReceipt {
    pub frame_id: RealtimeFrameId,
    pub presented: bool,
    pub tick: engine_core::Tick,
    pub frame_budget_micros: u32,
}

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
    pub fn new(world: WorldState, config: RealtimeRuntimeConfig) -> RealtimeRuntimeResult<Self> {
        if config.target_fps == 0 {
            return Err(RealtimeRuntimeFailure::for_reason(
                RealtimeRuntimeFailureReason::InvalidTargetFps,
            ));
        }
        if config.visibility_freshness_frames == 0 {
            return Err(RealtimeRuntimeFailure::for_reason(
                RealtimeRuntimeFailureReason::InvalidFrameCadence,
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

    pub fn step(&mut self) -> RealtimeRuntimeResult<RealtimeExecutionResult> {
        let frame_id = RealtimeFrameId(self.next_frame_id);
        self.next_frame_id = self.next_frame_id.saturating_add(1);
        if self.config.enqueue_presentable_frames {
            self.kernel
                .enqueue_presentable_frame(PresentableFrame {
                    frame_id: frame_id.0,
                    visibility_freshness_frames: self.config.visibility_freshness_frames,
                })
                .map_err(|_| {
                    RealtimeRuntimeFailure::for_reason(
                        RealtimeRuntimeFailureReason::InvalidRuntimeState,
                    )
                })?;
        }
        let kernel_result = self.kernel.run_tick().map_err(|_| {
            RealtimeRuntimeFailure::for_reason(RealtimeRuntimeFailureReason::InvalidRuntimeState)
        })?;
        Ok(RealtimeExecutionResult {
            frame_presented: kernel_result.presented_frame,
            frame_id,
            kernel_result,
        })
    }

    pub fn step_with_receipt(&mut self) -> RealtimeRuntimeResult<RealtimeFrameReceipt> {
        let cadence = self.cadence()?;
        let result = self.step()?;
        Ok(RealtimeFrameReceipt {
            frame_id: result.frame_id,
            presented: result.frame_presented,
            tick: result.kernel_result.tick,
            frame_budget_micros: cadence.frame_budget_micros,
        })
    }

    pub fn cadence(&self) -> RealtimeRuntimeResult<RealtimeFrameCadence> {
        RealtimeFrameCadence::from_fps(self.config.target_fps)
    }

    pub fn presentation_budget_decision(
        &self,
        frame_time_micros: u32,
    ) -> RealtimeRuntimeResult<PresentationBudgetDecision> {
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
