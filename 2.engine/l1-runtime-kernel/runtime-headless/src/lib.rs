use engine_runtime::{
    EngineCoreResult, ExecutionResult, RuntimeConfig, RuntimeKernel, RuntimeProfile,
};
use engine_world::WorldState;
use serde::{Deserialize, Serialize};

pub const HEADLESS_TICK_HZ: u16 = 20;
pub const HEADLESS_TICK_BUDGET_MS: u16 = 50;

/// Receipt confirming a headless run execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadlessRunReceipt {
    pub ticks_executed: u64,
    pub snapshot_bytes: usize,
    pub authoritative_window_ms: u64,
    pub replay_digest: HeadlessReplayDigest,
}

/// Receipt confirming a headless snapshot capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadlessSnapshotReceipt {
    pub snapshot_bytes: usize,
    pub segment_count: usize,
    pub tick: engine_core::Tick,
    pub replay_digest: HeadlessReplayDigest,
}

/// Digest of a headless replay for determinism verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeadlessReplayDigest(pub u64);

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

    /// Execute a headless run with receipt.
    pub fn run_with_receipt(&mut self, ticks: u64) -> EngineCoreResult<HeadlessRunReceipt> {
        for _ in 0..ticks {
            self.step()?;
        }
        let snapshot_bytes = self
            .kernel
            .world()
            .snapshot_bytes(self.config.snapshot_segment_count)?;
        Ok(HeadlessRunReceipt {
            ticks_executed: ticks,
            snapshot_bytes: snapshot_bytes.len(),
            authoritative_window_ms: ticks.saturating_mul(u64::from(HEADLESS_TICK_BUDGET_MS)),
            replay_digest: self.replay_digest()?,
        })
    }

    /// Capture snapshot with receipt.
    pub fn snapshot_with_receipt(&self) -> EngineCoreResult<HeadlessSnapshotReceipt> {
        let snapshot_bytes = self
            .kernel
            .world()
            .snapshot_bytes(self.config.snapshot_segment_count)?;
        Ok(HeadlessSnapshotReceipt {
            snapshot_bytes: snapshot_bytes.len(),
            segment_count: self.config.snapshot_segment_count,
            tick: self.kernel.world().current_tick(),
            replay_digest: self.replay_digest()?,
        })
    }

    /// Compute replay digest for determinism verification.
    pub fn replay_digest(&self) -> EngineCoreResult<HeadlessReplayDigest> {
        let snapshot_bytes = self
            .kernel
            .world()
            .snapshot_bytes(self.config.snapshot_segment_count)?;
        let world_digest = self.kernel.world().deterministic_digest()?;
        let runtime_digest = self.kernel.deterministic_digest()?;
        let mut digest = engine_core::StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.runtime.headless.replay")
            .write_u16(HEADLESS_TICK_HZ)
            .write_u16(HEADLESS_TICK_BUDGET_MS)
            .write_u64(self.kernel.world().current_tick().0)
            .write_u64(world_digest)
            .write_u64(runtime_digest)
            .write_u64(self.config.snapshot_segment_count as u64)
            .write_bool(self.config.emit_snapshot_bytes)
            .write_bytes(&snapshot_bytes);
        Ok(HeadlessReplayDigest(digest.finish().0))
    }
}
