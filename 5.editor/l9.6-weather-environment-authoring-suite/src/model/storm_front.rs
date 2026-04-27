use serde::{Deserialize, Serialize};

/// Parameters for creating or updating a storm front
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StormParams {
    /// Name of the storm front
    pub name: String,

    /// Center position [x, y, z]
    pub center_position: [f32; 3],

    /// Radius in meters
    pub radius: f32,

    /// Intensity (0.0 to 1.0)
    pub intensity: f32,

    /// Movement velocity [x, y, z] in m/s
    pub velocity: [f32; 3],

    /// Rain intensity in mm/hour
    pub rain_intensity_mm_per_hour: f32,

    /// Wind speed multiplier (1.0 = base wind speed)
    pub wind_speed_multiplier: f32,

    /// Lightning frequency (strikes per minute)
    pub lightning_frequency: f32,

    /// Whether the storm is active
    pub active: bool,
}

impl StormParams {
    pub fn new(name: String, center_position: [f32; 3]) -> Self {
        Self {
            name,
            center_position,
            radius: 1000.0,
            intensity: 0.5,
            velocity: [0.0, 0.0, 0.0],
            rain_intensity_mm_per_hour: 10.0,
            wind_speed_multiplier: 1.5,
            lightning_frequency: 0.0,
            active: true,
        }
    }

    /// Validate the storm parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Storm front name cannot be empty".to_string());
        }

        if self.radius <= 0.0 {
            return Err("Storm radius must be positive".to_string());
        }

        if self.radius > 100000.0 {
            return Err("Storm radius cannot exceed 100km".to_string());
        }

        if !(0.0..=1.0).contains(&self.intensity) {
            return Err("Storm intensity must be between 0.0 and 1.0".to_string());
        }

        if self.rain_intensity_mm_per_hour < 0.0 || self.rain_intensity_mm_per_hour > 200.0 {
            return Err("Rain intensity must be between 0.0 and 200.0 mm/hour".to_string());
        }

        if self.wind_speed_multiplier < 0.0 || self.wind_speed_multiplier > 10.0 {
            return Err("Wind speed multiplier must be between 0.0 and 10.0".to_string());
        }

        if self.lightning_frequency < 0.0 || self.lightning_frequency > 60.0 {
            return Err("Lightning frequency must be between 0.0 and 60.0 strikes/min".to_string());
        }

        Ok(())
    }
}

/// Storm front representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StormFront {
    pub params: StormParams,
    pub creation_time: u64,
    pub last_update_time: u64,
}

impl StormFront {
    pub fn new(params: StormParams) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            params,
            creation_time: now,
            last_update_time: now,
        }
    }

    pub fn update(&mut self, params: StormParams) {
        self.params = params;
        self.last_update_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}
