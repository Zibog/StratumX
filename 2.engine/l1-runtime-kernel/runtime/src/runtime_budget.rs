use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};

use crate::runtime_budget_policy::{
    axis_rank, bucket_code, bucket_rank, pressure_bucket, rung_for, threshold_row_id,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeMode {
    Headless,
    Realtime,
    Validation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PressureBucket {
    Green,
    Yellow,
    Orange,
    Red,
    HardFail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureAxis {
    Cpu,
    Gpu,
    Ram,
    Io,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegradeStep {
    LowerBroadphaseOrRouteDensity,
    LowerExpensiveAgentUpdateCadence,
    ReduceFarFieldFrequency,
    SampleNonAuthoritativeOverlays,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetEnvelope {
    pub scenario_id: String,
    pub baseline_id: String,
    pub compare_horizon_frames: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainBudgetUsage {
    pub cpu_frame_ms_x100: u32,
    pub gpu_frame_ms_x100: u32,
    pub ram_resident_mib: u32,
    pub io_hitch_ms_x100: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradeRung {
    pub axis: PressureAxis,
    pub threshold_code: String,
    pub degrade_step: DegradeStep,
    pub expected_recovery_trigger: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradeDecision {
    pub axis: PressureAxis,
    pub bucket: PressureBucket,
    pub threshold_row_id: String,
    pub compare_horizon_id: String,
    pub retained_baseline_artifact_id: String,
    pub first_blocking_code: String,
    pub next_legal_recovery_action: String,
    pub rung: Option<DegradeRung>,
}

impl BudgetEnvelope {
    pub fn validate(&self) -> EngineCoreResult<()> {
        if self.scenario_id.trim().is_empty() || self.baseline_id.trim().is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "budget envelope requires scenario and baseline ids",
            ));
        }
        if self.compare_horizon_frames == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "budget envelope requires non-zero compare horizon",
            ));
        }
        Ok(())
    }
}

pub(crate) fn build_degrade_decision(
    usage: &DomainBudgetUsage,
    envelope: &BudgetEnvelope,
) -> EngineCoreResult<DegradeDecision> {
    envelope.validate()?;
    let axes = [
        (
            PressureAxis::Cpu,
            pressure_bucket(usage.cpu_frame_ms_x100, 1250, 1400, 1550, 1660),
        ),
        (
            PressureAxis::Gpu,
            pressure_bucket(usage.gpu_frame_ms_x100, 1050, 1200, 1400, 1660),
        ),
        (
            PressureAxis::Ram,
            pressure_bucket(usage.ram_resident_mib, 4608, 5120, 5632, 6144),
        ),
        (
            PressureAxis::Io,
            pressure_bucket(usage.io_hitch_ms_x100, 400, 800, 1200, 1600),
        ),
    ];
    let (axis, bucket) = axes
        .into_iter()
        .max_by_key(|(axis, bucket)| (bucket_rank(*bucket), axis_rank(*axis)))
        .ok_or(EngineCoreError::InvalidDescriptor(
            "pressure decision requires axes",
        ))?;
    let threshold_row_id = threshold_row_id(axis).to_string();
    Ok(DegradeDecision {
        axis,
        bucket,
        threshold_row_id: threshold_row_id.clone(),
        compare_horizon_id: format!(
            "{}:{}f",
            envelope.scenario_id, envelope.compare_horizon_frames
        ),
        retained_baseline_artifact_id: envelope.baseline_id.clone(),
        first_blocking_code: format!("{}.{}", threshold_row_id, bucket_code(bucket)),
        next_legal_recovery_action: format!(
            "hold {} green frames before upward recovery",
            envelope.compare_horizon_frames
        ),
        rung: rung_for(
            axis,
            bucket,
            &threshold_row_id,
            envelope.compare_horizon_frames,
        ),
    })
}
