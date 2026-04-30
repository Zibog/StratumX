use crate::{DegradeRung, DegradeStep, PressureAxis, PressureBucket};

pub(crate) fn pressure_bucket(
    value: u32,
    green: u32,
    yellow: u32,
    orange: u32,
    red: u32,
) -> PressureBucket {
    match value {
        v if v <= green => PressureBucket::Green,
        v if v <= yellow => PressureBucket::Yellow,
        v if v <= orange => PressureBucket::Orange,
        v if v <= red => PressureBucket::Red,
        _ => PressureBucket::HardFail,
    }
}

pub(crate) fn threshold_row_id(axis: PressureAxis) -> &'static str {
    match axis {
        PressureAxis::Cpu => "pressure.cpu.frame_time",
        PressureAxis::Gpu => "pressure.gpu.frame_time",
        PressureAxis::Ram => "pressure.ram.residency",
        PressureAxis::Io => "pressure.io.hitch",
    }
}

pub(crate) fn rung_for(
    axis: PressureAxis,
    bucket: PressureBucket,
    threshold_code: &str,
    compare_horizon_frames: u32,
) -> Option<DegradeRung> {
    let degrade_step = match bucket {
        PressureBucket::Green => return None,
        PressureBucket::Yellow => DegradeStep::LowerBroadphaseOrRouteDensity,
        PressureBucket::Orange => DegradeStep::LowerExpensiveAgentUpdateCadence,
        PressureBucket::Red => DegradeStep::ReduceFarFieldFrequency,
        PressureBucket::HardFail => DegradeStep::SampleNonAuthoritativeOverlays,
    };
    Some(DegradeRung {
        axis,
        threshold_code: threshold_code.to_string(),
        degrade_step,
        expected_recovery_trigger: format!("{} green-frame horizon", compare_horizon_frames),
    })
}

pub(crate) fn bucket_rank(bucket: PressureBucket) -> u8 {
    match bucket {
        PressureBucket::Green => 0,
        PressureBucket::Yellow => 1,
        PressureBucket::Orange => 2,
        PressureBucket::Red => 3,
        PressureBucket::HardFail => 4,
    }
}

pub(crate) fn axis_rank(axis: PressureAxis) -> u8 {
    match axis {
        PressureAxis::Cpu => 4,
        PressureAxis::Gpu => 3,
        PressureAxis::Ram => 2,
        PressureAxis::Io => 1,
    }
}

pub(crate) fn bucket_code(bucket: PressureBucket) -> &'static str {
    match bucket {
        PressureBucket::Green => "green",
        PressureBucket::Yellow => "yellow",
        PressureBucket::Orange => "orange",
        PressureBucket::Red => "red",
        PressureBucket::HardFail => "hard_fail",
    }
}
