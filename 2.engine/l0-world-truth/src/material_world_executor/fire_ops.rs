// Fire operations - combustible objects

use super::executor::MaterialWorldExecutor;
use engine_core::{EngineCoreError, EngineCoreResult};
use engine_material::{
    BurnConsequenceReceipt, CombustibleMaterial, CombustibleObject, FireExposureContext,
};

impl MaterialWorldExecutor {
    pub fn create_combustible_object(
        &mut self,
        position: [f32; 3],
        material_type: CombustibleMaterial,
    ) -> usize {
        let obj = CombustibleObject::new(position, material_type);
        self.combustible_objects.push(obj);
        self.combustible_objects.len() - 1
    }

    pub fn apply_heat_to_object(
        &mut self,
        object_index: usize,
        heat_temp: f32,
        current_time: f32,
    ) -> bool {
        self.apply_fire_exposure_to_object(
            object_index,
            FireExposureContext {
                flame_temperature_celsius: heat_temp,
                exposure_duration_seconds: 1.0,
                oxygen_availability: 1.0,
            },
            current_time,
        )
        .map(|receipt| receipt.ignited)
        .unwrap_or(false)
    }

    pub fn apply_rain_to_object(&mut self, object_index: usize, intensity: f32, current_time: f32) {
        if let Some(obj) = self.combustible_objects.get_mut(object_index) {
            obj.apply_rain(intensity, current_time);
        }
    }

    pub fn get_combustible_object(&self, index: usize) -> Option<&CombustibleObject> {
        self.combustible_objects.get(index)
    }

    pub fn apply_fire_exposure_to_object(
        &mut self,
        object_index: usize,
        context: FireExposureContext,
        current_time: f32,
    ) -> EngineCoreResult<BurnConsequenceReceipt> {
        let obj = self.combustible_objects.get_mut(object_index).ok_or(
            EngineCoreError::InvalidDescriptor("combustible object index is out of range"),
        )?;
        let receipt = obj.apply_fire_exposure(&context, current_time)?;
        let event = receipt.to_material_consequence_event();
        self.material_events.push(event);
        self.burn_receipts.push(receipt.clone());
        Ok(receipt)
    }
}
