use engine_core::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetLatencyConfig {
    pub history_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyEstimate {
    pub round_trip_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LatencyBucket {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct JitterMeasure {
    pub variance_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyCompensation {
    pub prediction_window_ticks: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionContext {
    pub authoritative_tick: Tick,
    pub predicted_tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileResult {
    pub rewound: bool,
    pub delta_ticks: u64,
    pub correction: Option<ReconcileReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetLatencyMetrics {
    pub estimate: LatencyEstimate,
    pub bucket: LatencyBucket,
    pub jitter: JitterMeasure,
}

/// Prediction input for client-side prediction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionInput {
    pub tick: u64,
    pub input_data: Vec<u8>,
}

/// Prediction history tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionHistory {
    pub inputs: Vec<PredictionInput>,
    pub max_history: usize,
}

/// Rewind window for rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewindWindow {
    pub max_rewind_ticks: u64,
}

/// Reconcile receipt after server correction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileReceipt {
    pub corrected_tick: u64,
    pub delta_ticks: u64,
    pub reason: CorrectionReason,
    pub digest: u64,
}

/// Correction reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrectionReason {
    Misprediction,
    ServerAuthority,
    OutOfSync,
}

/// Prediction failure reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionRejectReason {
    NoPredictionHistory,
    OutsideRewindWindow,
}

impl PredictionHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            inputs: Vec::new(),
            max_history,
        }
    }

    pub fn add_input(&mut self, input: PredictionInput) {
        self.inputs.push(input);
        if self.inputs.len() > self.max_history {
            self.inputs.remove(0);
        }
    }

    pub fn validate_rewind(
        &self,
        tick: u64,
        window: &RewindWindow,
    ) -> Result<(), PredictionRejectReason> {
        if self.inputs.is_empty() {
            return Err(PredictionRejectReason::NoPredictionHistory);
        }
        let oldest_tick = self.inputs[0].tick;
        if tick < oldest_tick || tick > oldest_tick + window.max_rewind_ticks {
            return Err(PredictionRejectReason::OutsideRewindWindow);
        }
        Ok(())
    }
}

impl ReconcileReceipt {
    pub fn new(corrected_tick: u64, delta_ticks: u64, reason: CorrectionReason) -> Self {
        let mut builder = engine_core::StableDigestBuilder::new();
        builder.write_bytes(b"engine_net_latency.reconcile.v1");
        builder.write_u64(corrected_tick);
        builder.write_u64(delta_ticks);
        builder.write_u8(match reason {
            CorrectionReason::Misprediction => 1,
            CorrectionReason::ServerAuthority => 2,
            CorrectionReason::OutOfSync => 3,
        });
        Self {
            corrected_tick,
            delta_ticks,
            reason,
            digest: builder.finish().0,
        }
    }
}
