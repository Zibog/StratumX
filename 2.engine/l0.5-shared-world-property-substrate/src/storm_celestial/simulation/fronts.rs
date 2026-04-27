use super::super::storm_fronts::StormFrontState;

/// Create a new storm front and return its ID.
pub fn create_storm_front(
    storm_fronts: &mut Vec<StormFrontState>,
    next_storm_id: &mut u32,
    position: [f32; 3],
    velocity: [f32; 3],
    radius_km: f32,
    intensity: f32,
    rain_intensity_mm_per_hour: f32,
) -> u32 {
    let front_id = *next_storm_id;
    *next_storm_id += 1;
    storm_fronts.push(StormFrontState {
        front_id,
        position,
        velocity,
        radius_km,
        intensity: intensity.clamp(0.0, 1.0),
        rain_intensity_mm_per_hour: rain_intensity_mm_per_hour.max(0.0),
    });
    front_id
}

/// Update properties of an existing storm front.
pub fn update_storm_front(
    storm_fronts: &mut [StormFrontState],
    front_id: u32,
    position: Option<[f32; 3]>,
    velocity: Option<[f32; 3]>,
    radius_km: Option<f32>,
    intensity: Option<f32>,
    rain_intensity_mm_per_hour: Option<f32>,
) -> bool {
    if let Some(front) = storm_fronts.iter_mut().find(|f| f.front_id == front_id) {
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
        true
    } else {
        false
    }
}
