use super::WeatherEnvironmentAuthoringSuite;
use crate::model::{ClimateConfig, StormFront, StormFrontId, TheaterId, WeatherError};

impl WeatherEnvironmentAuthoringSuite {
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
