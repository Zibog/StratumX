use super::{CombustibleMaterial, FireState};
use crate::fire_weather::wetness::WetnessState;
use crate::{
    BurnAftermathPolicy, BurnConsequenceReceipt, FireExposureContext, MaterialId,
    PersistentBurnResult, WetnessModifier,
};
use engine_core::EngineCoreResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombustibleObject {
    pub position: [f32; 3],
    pub material_id: MaterialId,
    pub material_type: CombustibleMaterial,
    pub wetness: WetnessState,
    pub fire: FireState,
    pub persistent_burn_result: PersistentBurnResult,
}

impl CombustibleObject {
    pub fn new(position: [f32; 3], material_type: CombustibleMaterial) -> Self {
        Self {
            position,
            material_id: material_type.default_material_id(),
            material_type,
            wetness: WetnessState::new(),
            fire: FireState::new(),
            persistent_burn_result: PersistentBurnResult::Intact,
        }
    }

    pub fn can_ignite(&self) -> bool {
        let wetness_threshold = self.material_type.wetness_ignition_threshold();
        !self.fire.burning
            && self.wetness.wetness_percent < wetness_threshold
            && self.fire.fuel_remaining_percent > 0.0
    }

    pub fn apply_heat(&mut self, heat_temp: f32, current_time: f32) -> bool {
        if !self.can_ignite() {
            return false;
        }
        let ignition_temp = self.material_type.ignition_temperature_celsius();
        let drying_rate = self.material_type.drying_rate_percent_per_sec(heat_temp);
        self.wetness.dry(drying_rate, 1.0);
        self.can_ignite()
            && self
                .fire
                .attempt_ignition(heat_temp, ignition_temp, current_time)
    }

    pub fn apply_rain(&mut self, intensity: f32, current_time: f32) {
        self.wetness.add_water(intensity, current_time);
        if self.fire.burning && self.wetness.wetness_percent > 50.0 {
            self.fire.extinguish();
        }
    }

    pub fn apply_fire_exposure(
        &mut self,
        context: &FireExposureContext,
        current_time: f32,
    ) -> EngineCoreResult<BurnConsequenceReceipt> {
        context.validate()?;

        let wetness_modifier = WetnessModifier::from_wetness(self.wetness.wetness_percent);
        let ignition_temp = self.material_type.ignition_temperature_celsius()
            * wetness_modifier.ignition_threshold_multiplier;
        let drying_rate = self
            .material_type
            .drying_rate_percent_per_sec(context.flame_temperature_celsius)
            * context.oxygen_availability.max(0.1);
        self.wetness
            .dry(drying_rate, context.exposure_duration_seconds);

        let effective_flame_temp = context.flame_temperature_celsius * context.oxygen_availability;
        let ignited = self.can_ignite()
            && self
                .fire
                .attempt_ignition(effective_flame_temp, ignition_temp, current_time);
        if ignited {
            self.fire
                .update(context.exposure_duration_seconds * wetness_modifier.burn_rate_multiplier);
        }

        Ok(self.finalize_fire_exposure_receipt(ignited))
    }

    pub fn update(&mut self, delta_time: f32, ambient_temp: f32) {
        self.fire.update(delta_time);
        let drying_rate = self.material_type.drying_rate_percent_per_sec(ambient_temp);
        self.wetness.dry(drying_rate, delta_time);
    }

    fn finalize_fire_exposure_receipt(&mut self, ignited: bool) -> BurnConsequenceReceipt {
        let burn_family = self.material_type.burn_response_family();
        let aftermath_policy = BurnAftermathPolicy::for_burn_family(burn_family, ignited);
        let persistent_result = PersistentBurnResult::from_policy(aftermath_policy);
        if ignited {
            self.persistent_burn_result = persistent_result;
        }
        let fuel_consumed_percent = (100.0 - self.fire.fuel_remaining_percent).clamp(0.0, 100.0);

        BurnConsequenceReceipt::from_runtime(
            self.material_id,
            burn_family,
            ignited,
            fuel_consumed_percent,
            self.wetness.wetness_percent,
            aftermath_policy,
            self.persistent_burn_result,
        )
    }
}
