//! Storm command legality gates.

use crate::command_gates::common::{non_negative, non_zero_u32, normalized, positive, GateResult};
use crate::command_gates::verdict::LegalityVerdict;

pub fn validate_storm_create(
    radius_km: f32,
    intensity: f32,
    rain_intensity_mm_per_hour: f32,
) -> GateResult {
    positive(radius_km, "radius_km", "Storm radius")?;
    normalized(intensity, "intensity", "Storm intensity")?;
    non_negative(
        rain_intensity_mm_per_hour,
        "rain_intensity_mm_per_hour",
        "Rain intensity",
    )?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_storm_update(front_id: u32) -> GateResult {
    non_zero_u32(front_id, "front_id", "Storm front ID")?;
    Ok(LegalityVerdict::Legal)
}
