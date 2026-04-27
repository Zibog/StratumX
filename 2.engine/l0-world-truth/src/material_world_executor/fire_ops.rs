// Fire operations - combustible objects

use super::executor::MaterialWorldExecutor;
use engine_material::{CombustibleMaterial, CombustibleObject};

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
        if let Some(obj) = self.combustible_objects.get_mut(object_index) {
            obj.apply_heat(heat_temp, current_time)
        } else {
            false
        }
    }

    pub fn apply_rain_to_object(&mut self, object_index: usize, intensity: f32, current_time: f32) {
        if let Some(obj) = self.combustible_objects.get_mut(object_index) {
            obj.apply_rain(intensity, current_time);
        }
    }

    pub fn get_combustible_object(&self, index: usize) -> Option<&CombustibleObject> {
        self.combustible_objects.get(index)
    }
}
