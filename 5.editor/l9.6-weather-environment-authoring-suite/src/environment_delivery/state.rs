// Environment Authoring State

use editor_dto_law::{
    CloudPosture, FogPosture, PrecipitationPosture, ProfileRef, SkyEnvironmentBindingRef,
    SkyStateDto, StableWorldId,
};

use crate::weather_engine_bridge::WeatherEngineBridge;

pub struct EnvironmentAuthoringState {
    pub world_ref: Option<StableWorldId>,
    pub binding_ref: Option<SkyEnvironmentBindingRef>,
    pub profile_ref: Option<ProfileRef>,
    pub present: bool,
    pub time_of_day: f32,
    pub cloud_posture: CloudPosture,
    pub fog_posture: FogPosture,
    pub precipitation_posture: PrecipitationPosture,
    pub degraded: bool,
    pub engine_bridge: WeatherEngineBridge,
}

impl Default for EnvironmentAuthoringState {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentAuthoringState {
    pub fn new() -> Self {
        Self {
            world_ref: None,
            binding_ref: None,
            profile_ref: None,
            present: false,
            time_of_day: 12.0,
            cloud_posture: CloudPosture::Clear,
            fog_posture: FogPosture::None,
            precipitation_posture: PrecipitationPosture::None,
            degraded: false,
            engine_bridge: WeatherEngineBridge::new(),
        }
    }

    pub fn world_ref(&self) -> Option<StableWorldId> {
        self.world_ref
    }

    pub fn binding_ref(&self) -> Option<SkyEnvironmentBindingRef> {
        self.binding_ref
    }

    pub fn unbind(&mut self) {
        self.world_ref = None;
        self.binding_ref = None;
        self.profile_ref = None;
        self.present = false;
        self.degraded = false;
    }

    pub fn to_dto(&self) -> Option<SkyStateDto> {
        let world_ref = self.world_ref?;
        Some(SkyStateDto {
            world_ref,
            environment_binding_ref: self.binding_ref,
            present: self.present,
            time_of_day: self.time_of_day,
            date_or_cycle_ref: None,
            cloud_posture: self.cloud_posture.clone(),
            fog_posture: self.fog_posture.clone(),
            precipitation_posture: self.precipitation_posture.clone(),
            degraded: self.degraded,
        })
    }
}
