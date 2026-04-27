/// Set rain state on the weather system.
/// Returns (rain_enabled, rain_intensity_mm_per_hour).
pub fn set_rain(enabled: bool, intensity_mm_per_hour: f32) -> (bool, f32) {
    (enabled, intensity_mm_per_hour.max(0.0))
}
