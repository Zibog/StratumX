//! Material Executor - material binding operations
//!
//! All material profile binding operations (visual, acoustic, light, microdetail, weather, runtime).

use super::super::runtime::ToolingRuntime;
use super::super::types::*;

impl ToolingRuntime {
    /// Bind visual response to material profile
    pub fn bind_material_visual_response(
        &mut self,
        handle: ObjectHandle,
        visual_family: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("visual_response").cloned();
        obj.fields
            .insert("visual_response".to_string(), visual_family.clone());
        self.record_mutation_for_field(handle, "visual_response", Some(visual_family), old_value);
        Ok(())
    }

    /// Bind acoustic profile to material profile
    pub fn bind_material_acoustic_profile(
        &mut self,
        handle: ObjectHandle,
        acoustic: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("acoustic_profile").cloned();
        obj.fields
            .insert("acoustic_profile".to_string(), acoustic.clone());
        self.record_mutation_for_field(handle, "acoustic_profile", Some(acoustic), old_value);
        Ok(())
    }

    /// Bind light response to material profile
    pub fn bind_material_light_response(
        &mut self,
        handle: ObjectHandle,
        light: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("light_response").cloned();
        obj.fields
            .insert("light_response".to_string(), light.clone());
        self.record_mutation_for_field(handle, "light_response", Some(light), old_value);
        Ok(())
    }

    /// Bind microdetail profile to material profile
    pub fn bind_material_microdetail_profile(
        &mut self,
        handle: ObjectHandle,
        microdetail: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("microdetail_profile").cloned();
        obj.fields
            .insert("microdetail_profile".to_string(), microdetail.clone());
        self.record_mutation_for_field(handle, "microdetail_profile", Some(microdetail), old_value);
        Ok(())
    }

    /// Bind weather modulation to material profile
    pub fn bind_material_weather_modulation(
        &mut self,
        handle: ObjectHandle,
        weather: String,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("weather_modulation").cloned();
        obj.fields
            .insert("weather_modulation".to_string(), weather.clone());
        self.record_mutation_for_field(handle, "weather_modulation", Some(weather), old_value);
        Ok(())
    }

    /// Set cheap runtime rung for material profile
    pub fn set_material_cheap_runtime_rung(
        &mut self,
        handle: ObjectHandle,
        rung: u32,
    ) -> Result<(), ToolingError> {
        let obj = self.object_mut(handle)?;
        let old_value = obj.fields.get("cheap_runtime_rung").cloned();
        obj.fields
            .insert("cheap_runtime_rung".to_string(), rung.to_string());
        self.record_mutation_for_field(
            handle,
            "cheap_runtime_rung",
            Some(rung.to_string()),
            old_value,
        );
        Ok(())
    }
}
