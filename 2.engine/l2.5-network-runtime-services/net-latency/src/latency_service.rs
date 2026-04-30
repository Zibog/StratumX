use crate::{
    JitterMeasure, LatencyBucket, LatencyEstimate, NetLatencyConfig, NetLatencyMetrics,
    PredictionContext, ReconcileResult,
};
use engine_core::{EngineCoreError, EngineCoreResult};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct NetLatencyService {
    config: NetLatencyConfig,
    samples_ms: VecDeque<u32>,
}

impl NetLatencyService {
    pub fn new(config: NetLatencyConfig) -> EngineCoreResult<Self> {
        if config.history_size == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "latency history_size must be non-zero",
            ));
        }
        Ok(Self {
            config,
            samples_ms: VecDeque::new(),
        })
    }

    pub fn record_sample(&mut self, round_trip_ms: u32) {
        if self.samples_ms.len() >= self.config.history_size {
            self.samples_ms.pop_front();
        }
        self.samples_ms.push_back(round_trip_ms);
    }

    pub fn metrics(&self) -> NetLatencyMetrics {
        let count = self.samples_ms.len().max(1) as u32;
        let sum: u32 = self.samples_ms.iter().copied().sum();
        let estimate_ms = sum / count;
        let bucket = if estimate_ms <= 50 {
            LatencyBucket::Low
        } else if estimate_ms <= 120 {
            LatencyBucket::Medium
        } else {
            LatencyBucket::High
        };
        let jitter = if self.samples_ms.len() <= 1 {
            0
        } else {
            let min = self.samples_ms.iter().min().copied().unwrap_or(estimate_ms);
            let max = self.samples_ms.iter().max().copied().unwrap_or(estimate_ms);
            max.saturating_sub(min)
        };
        NetLatencyMetrics {
            estimate: LatencyEstimate {
                round_trip_ms: estimate_ms,
            },
            bucket,
            jitter: JitterMeasure {
                variance_ms: jitter,
            },
        }
    }

    pub fn reconcile(&self, context: PredictionContext) -> ReconcileResult {
        let delta_ticks = context
            .authoritative_tick
            .0
            .max(context.predicted_tick.0)
            .saturating_sub(context.authoritative_tick.0.min(context.predicted_tick.0));
        let rewound = context.predicted_tick != context.authoritative_tick;
        ReconcileResult {
            rewound,
            delta_ticks,
            correction: rewound.then(|| {
                crate::ReconcileReceipt::new(
                    context.authoritative_tick.0,
                    delta_ticks,
                    crate::CorrectionReason::Misprediction,
                )
            }),
        }
    }
}
