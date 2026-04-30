use engine_core::Tick;
use serde::{Deserialize, Serialize};

pub const APPLY_QUEUE_SEGMENT_CEILING: usize = 16_384;
pub const APPLY_QUEUE_AGGREGATE_CEILING: usize = 65_536;
pub const TRANSFER_COMPLETION_QUEUE_CEILING: usize = 4_096;
pub const CONNECTION_PUBLICATION_QUEUE_CEILING: usize = 1_024;
pub const CONNECTION_PUBLICATION_BYTES_CEILING: usize = 256 * 1024;
pub const PRESENTABLE_FRAME_QUEUE_CEILING: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeProfile {
    Interactive60,
    ListenHost60,
    Headless20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimePhase {
    Ingress,
    Read,
    Compute,
    Resource,
    AuthoritySync,
    Stage,
    Apply,
    Egress,
    Diagnostics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub profile: RuntimeProfile,
    pub max_apply_segments_per_tick: usize,
    pub publish_passes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub tick: Tick,
    pub phase: RuntimePhase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub tick: Tick,
    pub applied_segments: usize,
    pub published_records: usize,
    pub presented_frame: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnostics {
    pub last_phase: RuntimePhase,
    pub apply_queue_depth: usize,
    pub transfer_completion_queue_depth: usize,
    pub connection_count: usize,
    pub presentable_frame_depth: usize,
    pub last_budget_blocking_code: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConnectionKey(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferCompletion {
    pub transfer_id: u64,
    pub bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRecord {
    pub order: u64,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentableFrame {
    pub frame_id: u64,
    pub visibility_freshness_frames: u8,
}
