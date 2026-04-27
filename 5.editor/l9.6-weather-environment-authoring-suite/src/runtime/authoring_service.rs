use std::collections::HashMap;

use crate::model::{
    ClimateConfig, StormFront, StormFrontId, StormParams, TheaterId, WeatherError, WeatherState,
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeatherEnvironmentAuthoringSuite {
    next_theater_id: u64,
    next_storm_front_id: u64,
    theaters: HashMap<TheaterId, ClimateConfig>,
    storm_fronts: HashMap<StormFrontId, StormFront>,
    theater_storm_fronts: HashMap<TheaterId, Vec<StormFrontId>>,
    active_theater: Option<TheaterId>,
}

impl WeatherEnvironmentAuthoringSuite {
    pub fn new() -> Self {
        Self {
            next_theater_id: 1,
            next_storm_front_id: 1,
            theaters: HashMap::new(),
            storm_fronts: HashMap::new(),
            theater_storm_fronts: HashMap::new(),
            active_theater: None,
        }
    }

    /// Configure a climate theater with validation
    /// Requirement 5.2: THE Weather_Environment_Authoring_Suite SHALL provide climate theater operator surface
    /// beyond bundle path and storm front ids
    pub fn configure_climate_theater(
        &mut self,
        config: ClimateConfig,
    ) -> Result<TheaterId, WeatherError> {
        // Validate configuration
        config
            .validate()
            .map_err(WeatherError::InvalidConfiguration)?;

        // Create new theater
        let theater_id = TheaterId::new(self.next_theater_id);
        self.next_theater_id += 1;

        self.theaters.insert(theater_id, config);
        self.theater_storm_fronts.insert(theater_id, Vec::new());

        // Set as active if it's the first theater
        if self.active_theater.is_none() {
            self.active_theater = Some(theater_id);
        }

        Ok(theater_id)
    }

    /// Create a storm front with parameters
    /// Requirement 5.2: THE Weather_Environment_Authoring_Suite SHALL provide storm front creation
    pub fn create_storm_front(
        &mut self,
        params: StormParams,
    ) -> Result<StormFrontId, WeatherError> {
        // Validate parameters
        params.validate().map_err(WeatherError::StormFrontError)?;

        // Create new storm front
        let storm_front_id = StormFrontId::new(self.next_storm_front_id);
        self.next_storm_front_id += 1;

        let storm_front = StormFront::new(params);
        self.storm_fronts.insert(storm_front_id, storm_front);

        // Add to active theater if one exists
        if let Some(theater_id) = self.active_theater {
            self.theater_storm_fronts
                .get_mut(&theater_id)
                .unwrap()
                .push(storm_front_id);
        }

        Ok(storm_front_id)
    }

    /// Update a storm front with dynamic parameters
    /// Requirement 5.2: THE Weather_Environment_Authoring_Suite SHALL provide storm front updates
    pub fn update_storm_front(
        &mut self,
        id: StormFrontId,
        params: StormParams,
    ) -> Result<(), WeatherError> {
        // Validate parameters
        params.validate().map_err(WeatherError::StormFrontError)?;

        // Check if storm front exists
        let storm_front = self
            .storm_fronts
            .get_mut(&id)
            .ok_or(WeatherError::UnknownStormFront)?;

        // Update storm front
        storm_front.update(params);

        Ok(())
    }

    /// Get current weather state
    /// Requirement 5.2: THE Weather_Environment_Authoring_Suite SHALL provide weather state access
    pub fn get_weather_state(&self) -> WeatherState {
        let mut state = WeatherState::new();

        // Get active theater configuration
        if let Some(theater_id) = self.active_theater {
            if let Some(config) = self.theaters.get(&theater_id) {
                // Apply base configuration
                state.temperature_celsius = config.base_temperature_celsius;
                state.humidity = config.humidity;
                state.wind_speed = config.wind_base_speed;
                state.cloud_coverage = config.cloud_coverage;
                state.fog_density = config.fog_density;
                state.time_of_day_hours = config.time_of_day_hours;

                // Calculate sun elevation (simplified)
                let hour_angle = (config.time_of_day_hours - 12.0) * 15.0;
                state.sun_elevation_deg = config.latitude_deg + hour_angle.sin() * 23.5;

                // Get storm fronts for this theater
                if let Some(storm_front_ids) = self.theater_storm_fronts.get(&theater_id) {
                    let active_storms: Vec<_> = storm_front_ids
                        .iter()
                        .filter_map(|id| self.storm_fronts.get(id))
                        .filter(|sf| sf.params.active)
                        .collect();

                    state.active_storm_count = active_storms.len();

                    // Apply storm effects
                    for storm in active_storms {
                        state.precipitation_rate += storm.params.rain_intensity_mm_per_hour;
                        state.wind_speed += config.wind_base_speed
                            * (storm.params.wind_speed_multiplier - 1.0)
                            * storm.params.intensity;
                        state.cloud_coverage =
                            (state.cloud_coverage + storm.params.intensity * 0.3).min(1.0);

                        if storm.params.lightning_frequency > 0.0 {
                            state.lightning_active = true;
                        }
                    }

                    state.is_raining = state.precipitation_rate > 0.0;

                    // Reduce visibility based on precipitation
                    if state.is_raining {
                        state.visibility_meters =
                            10000.0 * (1.0 - (state.precipitation_rate / 100.0).min(0.9));
                    }
                }
            }
        }

        state
    }

    // Helper methods for testing and inspection

    pub fn get_theater(&self, theater_id: TheaterId) -> Option<&ClimateConfig> {
        self.theaters.get(&theater_id)
    }

    pub fn get_storm_front(&self, storm_front_id: StormFrontId) -> Option<&StormFront> {
        self.storm_fronts.get(&storm_front_id)
    }

    pub fn get_theater_storm_fronts(&self, theater_id: TheaterId) -> Vec<StormFrontId> {
        self.theater_storm_fronts
            .get(&theater_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_active_theater(&mut self, theater_id: TheaterId) -> Result<(), WeatherError> {
        if !self.theaters.contains_key(&theater_id) {
            return Err(WeatherError::UnknownTheater);
        }
        self.active_theater = Some(theater_id);
        Ok(())
    }

    pub fn get_active_theater(&self) -> Option<TheaterId> {
        self.active_theater
    }

    pub fn theater_count(&self) -> usize {
        self.theaters.len()
    }

    pub fn storm_front_count(&self) -> usize {
        self.storm_fronts.len()
    }
}

impl Default for WeatherEnvironmentAuthoringSuite {
    fn default() -> Self {
        Self::new()
    }
}
