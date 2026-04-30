// Material validation rules

use crate::validation::errors::ValidationResult;

/// Validate material duplicate parameters
pub fn validate_material_duplicate(
    source_material_id: &str,
    target_name: &str,
) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if source_material_id.is_empty() {
        result = result.add_error("source_material_id cannot be empty");
    }
    if target_name.is_empty() {
        result = result.add_error("target_name cannot be empty");
    }
    result
}

/// Validate material binding parameters
pub fn validate_material_binding(material_id: &str, binding_value: &str) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if material_id.is_empty() {
        result = result.add_error("material_id cannot be empty");
    }
    if binding_value.is_empty() {
        result = result.add_error("binding value cannot be empty");
    }
    result
}

/// Validate material rung level
pub fn validate_material_rung(material_id: &str, rung_level: u32) -> ValidationResult {
    let mut result = ValidationResult::Valid;
    if material_id.is_empty() {
        result = result.add_error("material_id cannot be empty");
    }
    if rung_level > 10 {
        result = result.add_error("rung_level cannot exceed 10");
    }
    result
}
