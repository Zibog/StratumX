/// Set fog density and compute horizon visibility.
/// Returns (fog_density, horizon_visibility_km).
pub fn set_fog_density(value: f32) -> (f32, f32) {
    let clamped = value.clamp(0.0, 1.0);
    let horizon_visibility_km = (1.0 - clamped) * 100.0;
    (clamped, horizon_visibility_km)
}
