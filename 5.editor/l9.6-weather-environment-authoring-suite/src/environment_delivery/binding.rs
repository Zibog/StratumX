// Environment Binding Operations

use super::state::EnvironmentAuthoringState;
use editor_dto_law::{
    CloudPosture, FogPosture, PrecipitationPosture, ProfileRef, SkyEnvironmentBindingRef,
    StableWorldId,
};
use uuid::Uuid;

impl EnvironmentAuthoringState {
    pub fn bind_to_world(
        &mut self,
        world_ref: StableWorldId,
        profile_ref: ProfileRef,
    ) -> Result<SkyEnvironmentBindingRef, String> {
        if self.world_ref.is_some() && self.world_ref != Some(world_ref) {
            return Err("Environment already bound to different world".to_string());
        }

        let binding_ref = SkyEnvironmentBindingRef(Uuid::new_v4());
        self.world_ref = Some(world_ref);
        self.binding_ref = Some(binding_ref);
        self.profile_ref = Some(profile_ref);
        self.present = true;

        Ok(binding_ref)
    }

    pub fn set_time_of_day(&mut self, time: f32) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        if !(0.0..=24.0).contains(&time) {
            return Err("Time must be between 0.0 and 24.0".to_string());
        }
        self.time_of_day = time;
        Ok(())
    }

    pub fn set_environment_state(
        &mut self,
        cloud: CloudPosture,
        fog: FogPosture,
        precipitation: PrecipitationPosture,
    ) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.cloud_posture = cloud;
        self.fog_posture = fog;
        self.precipitation_posture = precipitation;
        Ok(())
    }

    pub fn set_fog_precipitation(
        &mut self,
        fog: FogPosture,
        precipitation: PrecipitationPosture,
    ) -> Result<(), String> {
        if self.binding_ref.is_none() {
            return Err("No environment binding exists".to_string());
        }
        self.fog_posture = fog;
        self.precipitation_posture = precipitation;
        Ok(())
    }
}
