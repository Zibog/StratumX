//! Sky and weather command legality gates.

use crate::command_gates::common::{invalid, non_negative, normalized, positive, GateResult};
use crate::command_gates::verdict::LegalityVerdict;

pub fn validate_sky_set_time_of_day(hours: f32) -> GateResult {
    if !(0.0..24.0).contains(&hours) {
        return Err(invalid(
            "hours",
            format!("Time of day must be 0-24, got {hours}"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_day_of_year(day: u16) -> GateResult {
    if !(1..=365).contains(&day) {
        return Err(invalid(
            "day",
            format!("Day of year must be 1-365, got {day}"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_latitude(latitude_deg: f32) -> GateResult {
    if !(-90.0..=90.0).contains(&latitude_deg) {
        return Err(invalid(
            "latitude_deg",
            format!("Latitude must be -90 to 90, got {latitude_deg}"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_normalized_value(field: &str, value: f32) -> GateResult {
    normalized(value, field, field)?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_rain(intensity_mm_per_hour: f32) -> GateResult {
    non_negative(
        intensity_mm_per_hour,
        "intensity_mm_per_hour",
        "Rain intensity",
    )?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_step_simulation(dt_seconds: f32) -> GateResult {
    positive(dt_seconds, "dt_seconds", "Simulation delta time")?;
    if dt_seconds > 3600.0 {
        return Err(invalid(
            "dt_seconds",
            format!("Simulation delta time too large: {dt_seconds} > 3600"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}
