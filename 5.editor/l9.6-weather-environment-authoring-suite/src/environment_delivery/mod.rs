// Environment Delivery - Modular Structure

pub mod binding;
pub mod diagnostics;
pub mod simulation;
pub mod state;
pub mod sun_profile;
pub mod weather_cells;
pub mod weather_controls;
pub mod world_sync;

pub use diagnostics::*;
pub use state::*;

use editor_dto_law::{
    CloudPosture, CloudShadowPosture, FogPosture, PrecipitationPosture, ProfileRef,
    SkyEnvironmentBindingRef, SkyStateDto, StableWorldId, WeatherRegime, WeatherStateSnapshot,
};
use engine_world::WorldState;

pub struct EnvironmentDeliveryThread {
    state: EnvironmentAuthoringState,
}

impl EnvironmentDeliveryThread {
    pub fn new() -> Self {
        Self {
            state: EnvironmentAuthoringState::new(),
        }
    }

    pub fn profile_binding(
        &mut self,
        world_ref: StableWorldId,
        profile_ref: ProfileRef,
    ) -> Result<SkyEnvironmentBindingRef, String> {
        self.state.bind_to_world(world_ref, profile_ref)
    }

    pub fn time_of_day(&mut self, time: f32) -> Result<(), String> {
        self.state.set_time_of_day(time)
    }

    pub fn environment_state(
        &mut self,
        cloud: CloudPosture,
        fog: FogPosture,
        precipitation: PrecipitationPosture,
    ) -> Result<(), String> {
        self.state.set_environment_state(cloud, fog, precipitation)
    }

    pub fn fog_precipitation(
        &mut self,
        fog: FogPosture,
        precipitation: PrecipitationPosture,
    ) -> Result<(), String> {
        self.state.set_fog_precipitation(fog, precipitation)
    }

    pub fn sun_profile_binding(&mut self) -> Result<(), String> {
        self.state.bind_sun_profile()
    }

    pub fn set_sun_temperature(&mut self, kelvin: f32) -> Result<(), String> {
        self.state.set_sun_temperature(kelvin)
    }

    pub fn set_sun_halo(&mut self, intensity: f32, falloff: f32) -> Result<(), String> {
        self.state.set_sun_halo(intensity, falloff)
    }

    pub fn set_sun_disk_radius(&mut self, angular_radius_deg: f32) -> Result<(), String> {
        self.state.set_sun_disk_radius(angular_radius_deg)
    }

    pub fn set_sun_disk_softness(&mut self, softness: f32) -> Result<(), String> {
        self.state.set_sun_disk_softness(softness)
    }

    pub fn set_sun_pulse_curve(&mut self, pulse: Option<f32>) -> Result<(), String> {
        self.state.set_sun_pulse_curve(pulse)
    }

    pub fn set_weather_regime(&mut self, regime: WeatherRegime) -> Result<(), String> {
        self.state.set_weather_regime(regime)
    }

    pub fn lock_weather_authoring(&mut self, locked: bool) -> Result<(), String> {
        self.state.lock_weather_authoring(locked)
    }

    pub fn set_weather_bias(&mut self, storm: f32, fog: f32, rain: f32) -> Result<(), String> {
        self.state.set_weather_bias(storm, fog, rain)
    }

    pub fn create_weather_cell(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        density: f32,
    ) -> Result<u32, String> {
        self.state
            .create_weather_cell(position, velocity, radius_km, density)
    }

    pub fn update_weather_cell(
        &mut self,
        cell_id: u32,
        precipitation_rate: Option<f32>,
        lightning_probability: Option<f32>,
    ) -> Result<(), String> {
        self.state
            .update_weather_cell(cell_id, precipitation_rate, lightning_probability)
    }

    pub fn create_storm_cell(&mut self, position: [f32; 3], intensity: f32) -> Result<u32, String> {
        self.state.create_storm_cell(position, intensity)
    }

    pub fn set_cloud_shadow_active(&mut self, active: bool) -> Result<(), String> {
        self.state.set_cloud_shadow_active(active)
    }

    pub fn set_cloud_shadow_posture(&mut self, posture: CloudShadowPosture) -> Result<(), String> {
        self.state.set_cloud_shadow_posture(posture)
    }

    pub fn apply_weather_scenario(
        &mut self,
        scenario: &crate::WeatherScenario,
    ) -> Result<(), String> {
        self.state.apply_weather_scenario(scenario)
    }

    pub fn step_simulation(&mut self, delta_time: f32) {
        self.state.step_simulation(delta_time);
    }

    pub fn sync_to_world(&mut self, world: &mut WorldState) -> Result<(), String> {
        self.state.sync_to_world(world)
    }

    pub fn sync_from_world(&mut self, world: &WorldState) -> Result<(), String> {
        self.state.sync_from_world(world)
    }

    pub fn diagnostics(&self) -> EnvironmentDiagnostics {
        self.state.diagnostics()
    }

    pub fn get_state_dto(&self) -> Option<SkyStateDto> {
        self.state.to_dto()
    }

    pub fn get_weather_snapshot(&self) -> Option<WeatherStateSnapshot> {
        self.state.weather_snapshot()
    }

    pub fn get_world_ref(&self) -> Option<StableWorldId> {
        self.state.world_ref()
    }

    pub fn get_binding_ref(&self) -> Option<SkyEnvironmentBindingRef> {
        self.state.binding_ref()
    }

    pub fn unbind(&mut self) {
        self.state.unbind();
    }
}

impl Default for EnvironmentDeliveryThread {
    fn default() -> Self {
        Self::new()
    }
}
