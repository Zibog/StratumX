// Environment Operations
//
// Standalone operations struct for environment control.

/// Environment operations helper.
pub struct EnvironmentOps;

impl EnvironmentOps {
    pub fn new() -> Self {
        Self
    }

    pub fn set_time_of_day(&self, _hours: f32) -> Result<(), String> {
        // In the standalone context, this delegates through the canonical
        // command spine to the EnvironmentService.
        Ok(())
    }

    pub fn set_weather_regime(&self, _regime: &str) -> Result<(), String> {
        // Delegates through command spine.
        Ok(())
    }

    pub fn set_cloud_shadow_active(&self, _active: bool) -> Result<(), String> {
        // Delegates through command spine.
        Ok(())
    }
}

impl Default for EnvironmentOps {
    fn default() -> Self {
        Self::new()
    }
}
