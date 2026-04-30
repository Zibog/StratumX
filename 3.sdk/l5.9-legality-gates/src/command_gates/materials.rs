//! Material and physical-response command legality gates.
//!
//! This module contains material, terrain, and destruction-facing legality
//! checks extracted from the legacy SDK command gate monolith.

use super::common::{
    invalid, max_len, non_empty, non_negative, non_zero_u16, non_zero_u32, normalized, positive,
    GateResult,
};
use super::verdict::LegalityVerdict;

pub fn validate_material_create_archetype(
    label: &str,
    hardness_mohs: f32,
    density_kg_m3: f32,
) -> GateResult {
    non_empty(label, "label", "Material archetype label cannot be empty")?;
    if !(0.0..=10.0).contains(&hardness_mohs) {
        return Err(invalid(
            "hardness_mohs",
            format!("Hardness must be 0-10, got {hardness_mohs}"),
        ));
    }
    positive(density_kg_m3, "density_kg_m3", "Density")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_create_stack(label: &str) -> GateResult {
    non_empty(label, "label", "Material stack label cannot be empty")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_stack_add_layer(stack_id: u16, thickness_mm: f32) -> GateResult {
    non_zero_u16(stack_id, "stack_id", "Stack ID")?;
    positive(thickness_mm, "thickness_mm", "Thickness")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_stack_remove_layer(stack_id: u16, layer_index: u8) -> GateResult {
    non_zero_u16(stack_id, "stack_id", "Stack ID")?;
    if layer_index >= 8 {
        return Err(invalid(
            "layer_index",
            format!("Layer index must be 0-7, got {layer_index}"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_assign_stack(entity_id: u32, stack_id: u16) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    non_zero_u16(stack_id, "stack_id", "Stack ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_terrain_create_patch(size: [f32; 2], label: &str) -> GateResult {
    if size[0] <= 0.0 || size[1] <= 0.0 {
        return Err(invalid(
            "size",
            format!(
                "Terrain size must be positive, got [{}, {}]",
                size[0], size[1]
            ),
        ));
    }
    non_empty(label, "label", "Terrain patch label cannot be empty")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_terrain_paint_surface(
    target_entity_id: u32,
    radius: f32,
    stack_id: u16,
) -> GateResult {
    non_zero_u32(target_entity_id, "target_entity_id", "Target entity ID")?;
    positive(radius, "radius", "Paint radius")?;
    non_zero_u16(stack_id, "stack_id", "Stack ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_set_terrain_material(material_type: &str) -> GateResult {
    non_empty(
        material_type,
        "material_type",
        "Material type cannot be empty",
    )?;
    max_len(material_type, 128, "material_type", "Material type")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_trigger_blast(energy_j: f32) -> GateResult {
    positive(energy_j, "energy_j", "Blast energy")?;
    if energy_j > 1_000_000_000.0 {
        return Err(invalid(
            "energy_j",
            format!("Blast energy excessively large: {energy_j} > 1e9"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_set_integrity(integrity: f32) -> GateResult {
    normalized(integrity, "integrity", "Integrity")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_set_support_type(structure_type: &str) -> GateResult {
    non_empty(
        structure_type,
        "structure_type",
        "Structure type cannot be empty",
    )?;
    max_len(structure_type, 128, "structure_type", "Structure type")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_apply_support_damage(
    energy_j: f32,
    impact_direction: [f32; 3],
) -> GateResult {
    non_negative(energy_j, "energy_j", "Damage energy")?;
    let magnitude = (impact_direction[0] * impact_direction[0]
        + impact_direction[1] * impact_direction[1]
        + impact_direction[2] * impact_direction[2])
        .sqrt();
    if magnitude < 0.001 {
        return Err(invalid(
            "impact_direction",
            "Impact direction must have non-negligible magnitude",
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_reset_state() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_set_barrel_water(liters: f32) -> GateResult {
    non_negative(liters, "liters", "Water volume")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_set_barrel_leak() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_ignite_fire() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_extinguish_fire() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_set_wetness(wetness_percent: f32) -> GateResult {
    if !(0.0..=100.0).contains(&wetness_percent) {
        return Err(invalid(
            "wetness_percent",
            format!("Wetness must be 0-100, got {wetness_percent}"),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_update(delta_time: f32) -> GateResult {
    positive(delta_time, "delta_time", "Delta time")?;
    Ok(LegalityVerdict::Legal)
}
