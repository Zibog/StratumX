use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StormFrontState {
    pub front_id: u32,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub radius_km: f32,
    pub intensity: f32,
    pub rain_intensity_mm_per_hour: f32,
}

pub fn create_storm_front(
    front_id: u32,
    position: [f32; 3],
    velocity: [f32; 3],
    radius_km: f32,
    intensity: f32,
    rain_intensity_mm_per_hour: f32,
) -> StormFrontState {
    StormFrontState {
        front_id,
        position,
        velocity,
        radius_km,
        intensity: intensity.clamp(0.0, 1.0),
        rain_intensity_mm_per_hour: rain_intensity_mm_per_hour.max(0.0),
    }
}

pub fn update_storm_front(
    front: &mut StormFrontState,
    position: Option<[f32; 3]>,
    velocity: Option<[f32; 3]>,
    radius_km: Option<f32>,
    intensity: Option<f32>,
    rain_intensity_mm_per_hour: Option<f32>,
) {
    if let Some(pos) = position {
        front.position = pos;
    }
    if let Some(vel) = velocity {
        front.velocity = vel;
    }
    if let Some(rad) = radius_km {
        front.radius_km = rad;
    }
    if let Some(int) = intensity {
        front.intensity = int.clamp(0.0, 1.0);
    }
    if let Some(rain) = rain_intensity_mm_per_hour {
        front.rain_intensity_mm_per_hour = rain.max(0.0);
    }
}

pub fn step_storm_front(front: &mut StormFrontState, dt_seconds: f32) {
    front.position[0] += front.velocity[0] * dt_seconds;
    front.position[1] += front.velocity[1] * dt_seconds;
    front.position[2] += front.velocity[2] * dt_seconds;
}
