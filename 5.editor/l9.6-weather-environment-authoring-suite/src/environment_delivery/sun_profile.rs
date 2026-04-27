// Sun Profile Operations

use super::state::EnvironmentAuthoringState;

impl EnvironmentAuthoringState {
    pub fn bind_sun_profile(&mut self) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        Ok(())
    }

    pub fn set_sun_temperature(&mut self, kelvin: f32) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_sun_temperature(kelvin)
    }

    pub fn set_sun_halo(&mut self, intensity: f32, falloff: f32) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_sun_halo(intensity, falloff)
    }

    pub fn set_sun_disk_radius(&mut self, angular_radius_deg: f32) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge.set_sun_disk_radius(angular_radius_deg)
    }

    pub fn set_sun_disk_softness(&mut self, softness: f32) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge
            .sky_state_mut()
            .set_sun_disk_softness(softness);
        Ok(())
    }

    pub fn set_sun_pulse_curve(&mut self, pulse: Option<f32>) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.engine_bridge
            .sky_state_mut()
            .set_sun_pulse_curve(pulse);
        Ok(())
    }
}
