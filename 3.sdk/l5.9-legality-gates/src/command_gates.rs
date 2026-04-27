//! Command-specific legality gates
//!
//! Provides validation functions for all ingress packet command types.

use sdk_compat::verdicts::{LegalityRejection, LegalityRejectionReason, LegalityVerdict};

// ============================================================================
// SCENE COMMAND GATES
// ============================================================================

pub fn validate_scene_create_empty(scene_name: &str) -> Result<LegalityVerdict, LegalityRejection> {
    if scene_name.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "scene_name".into(),
                message: "Scene name cannot be empty".into(),
            },
        });
    }
    if scene_name.len() > 256 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "scene_name".into(),
                message: format!("Scene name too long: {} > 256", scene_name.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_create_entity_from_asset(
    asset_id: u32,
    label: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if asset_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "asset_id".into(),
                message: "Asset ID cannot be zero".into(),
            },
        });
    }
    if label.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "label".into(),
                message: "Entity label cannot be empty".into(),
            },
        });
    }
    if label.len() > 256 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "label".into(),
                message: format!("Entity label too long: {} > 256", label.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_set_transform(entity_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_delete_entity(entity_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_get_entity_details(
    entity_id: u32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// MATERIAL COMMAND GATES
// ============================================================================

pub fn validate_material_create_archetype(
    label: &str,
    hardness_mohs: f32,
    density_kg_m3: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if label.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "label".into(),
                message: "Material archetype label cannot be empty".into(),
            },
        });
    }
    if !(0.0..=10.0).contains(&hardness_mohs) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "hardness_mohs".into(),
                message: format!("Hardness must be 0-10, got {}", hardness_mohs),
            },
        });
    }
    if density_kg_m3 <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "density_kg_m3".into(),
                message: format!("Density must be positive, got {}", density_kg_m3),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_create_stack(label: &str) -> Result<LegalityVerdict, LegalityRejection> {
    if label.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "label".into(),
                message: "Material stack label cannot be empty".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_stack_add_layer(
    stack_id: u16,
    thickness_mm: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if stack_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "stack_id".into(),
                message: "Stack ID cannot be zero".into(),
            },
        });
    }
    if thickness_mm <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "thickness_mm".into(),
                message: format!("Thickness must be positive, got {}", thickness_mm),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_stack_remove_layer(
    stack_id: u16,
    layer_index: u8,
) -> Result<LegalityVerdict, LegalityRejection> {
    if stack_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "stack_id".into(),
                message: "Stack ID cannot be zero".into(),
            },
        });
    }
    if layer_index >= 8 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "layer_index".into(),
                message: format!("Layer index must be 0-7, got {}", layer_index),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_assign_stack(
    entity_id: u32,
    stack_id: u16,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if stack_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "stack_id".into(),
                message: "Stack ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// TERRAIN COMMAND GATES
// ============================================================================

pub fn validate_terrain_create_patch(
    size: [f32; 2],
    label: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if size[0] <= 0.0 || size[1] <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "size".into(),
                message: format!(
                    "Terrain size must be positive, got [{}, {}]",
                    size[0], size[1]
                ),
            },
        });
    }
    if label.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "label".into(),
                message: "Terrain patch label cannot be empty".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_terrain_paint_surface(
    target_entity_id: u32,
    radius: f32,
    stack_id: u16,
) -> Result<LegalityVerdict, LegalityRejection> {
    if target_entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "target_entity_id".into(),
                message: "Target entity ID cannot be zero".into(),
            },
        });
    }
    if radius <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "radius".into(),
                message: format!("Paint radius must be positive, got {}", radius),
            },
        });
    }
    if stack_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "stack_id".into(),
                message: "Stack ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// ACTOR COMMAND GATES
// ============================================================================

pub fn validate_actor_spawn_preset(preset_id: u16) -> Result<LegalityVerdict, LegalityRejection> {
    if preset_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "preset_id".into(),
                message: "Preset ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_actor_attach_weapon(
    actor_id: u32,
    weapon_profile_id: u16,
) -> Result<LegalityVerdict, LegalityRejection> {
    if actor_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "actor_id".into(),
                message: "Actor ID cannot be zero".into(),
            },
        });
    }
    if weapon_profile_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "weapon_profile_id".into(),
                message: "Weapon profile ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_actor_set_active(actor_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if actor_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "actor_id".into(),
                message: "Actor ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// ASSET COMMAND GATES
// ============================================================================

pub fn validate_asset_import(
    path: &str,
    label: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if path.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "path".into(),
                message: "Asset path cannot be empty".into(),
            },
        });
    }
    if label.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "label".into(),
                message: "Asset label cannot be empty".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_asset_get_details(asset_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if asset_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "asset_id".into(),
                message: "Asset ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// SKY/WEATHER COMMAND GATES
// ============================================================================

pub fn validate_sky_set_time_of_day(hours: f32) -> Result<LegalityVerdict, LegalityRejection> {
    if !(0.0..24.0).contains(&hours) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "hours".into(),
                message: format!("Time of day must be 0-24, got {}", hours),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_day_of_year(day: u16) -> Result<LegalityVerdict, LegalityRejection> {
    if !(1..=365).contains(&day) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "day".into(),
                message: format!("Day of year must be 1-365, got {}", day),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_latitude(latitude_deg: f32) -> Result<LegalityVerdict, LegalityRejection> {
    if !(-90.0..=90.0).contains(&latitude_deg) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "latitude_deg".into(),
                message: format!("Latitude must be -90 to 90, got {}", latitude_deg),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_normalized_value(
    field: &str,
    value: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if !(0.0..=1.0).contains(&value) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: field.into(),
                message: format!("{} must be 0-1, got {}", field, value),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_set_rain(
    intensity_mm_per_hour: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if intensity_mm_per_hour < 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "intensity_mm_per_hour".into(),
                message: format!(
                    "Rain intensity cannot be negative, got {}",
                    intensity_mm_per_hour
                ),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_sky_step_simulation(dt_seconds: f32) -> Result<LegalityVerdict, LegalityRejection> {
    if dt_seconds <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "dt_seconds".into(),
                message: format!("Simulation delta time must be positive, got {}", dt_seconds),
            },
        });
    }
    if dt_seconds > 3600.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "dt_seconds".into(),
                message: format!("Simulation delta time too large: {} > 3600", dt_seconds),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_storm_create(
    radius_km: f32,
    intensity: f32,
    rain_intensity_mm_per_hour: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if radius_km <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "radius_km".into(),
                message: format!("Storm radius must be positive, got {}", radius_km),
            },
        });
    }
    if !(0.0..=1.0).contains(&intensity) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "intensity".into(),
                message: format!("Storm intensity must be 0-1, got {}", intensity),
            },
        });
    }
    if rain_intensity_mm_per_hour < 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "rain_intensity_mm_per_hour".into(),
                message: format!(
                    "Rain intensity cannot be negative, got {}",
                    rain_intensity_mm_per_hour
                ),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_storm_update(front_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if front_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "front_id".into(),
                message: "Storm front ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// BALLISTICS COMMAND GATES
// ============================================================================

pub fn validate_ballistics_fire_active_actor() -> Result<LegalityVerdict, LegalityRejection> {
    // No specific validation needed for this command
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// DESTRUCTION COMMAND GATES
// ============================================================================

pub fn validate_destruction_set_terrain_material(
    material_type: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if material_type.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "material_type".into(),
                message: "Material type cannot be empty".into(),
            },
        });
    }
    if material_type.len() > 128 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "material_type".into(),
                message: format!("Material type too long: {} > 128", material_type.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_trigger_blast(
    energy_j: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if energy_j <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "energy_j".into(),
                message: format!("Blast energy must be positive, got {}", energy_j),
            },
        });
    }
    if energy_j > 1_000_000_000.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "energy_j".into(),
                message: format!("Blast energy excessively large: {} > 1e9", energy_j),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_set_integrity(
    integrity: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if !(0.0..=1.0).contains(&integrity) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "integrity".into(),
                message: format!("Integrity must be 0-1, got {}", integrity),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_set_support_type(
    structure_type: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if structure_type.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "structure_type".into(),
                message: "Structure type cannot be empty".into(),
            },
        });
    }
    if structure_type.len() > 128 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "structure_type".into(),
                message: format!("Structure type too long: {} > 128", structure_type.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_apply_support_damage(
    energy_j: f32,
    impact_direction: [f32; 3],
) -> Result<LegalityVerdict, LegalityRejection> {
    if energy_j < 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "energy_j".into(),
                message: format!("Damage energy cannot be negative, got {}", energy_j),
            },
        });
    }
    let magnitude = (impact_direction[0] * impact_direction[0]
        + impact_direction[1] * impact_direction[1]
        + impact_direction[2] * impact_direction[2])
        .sqrt();
    if magnitude < 0.001 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "impact_direction".into(),
                message: "Impact direction must have non-negligible magnitude".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_destruction_reset_state() -> Result<LegalityVerdict, LegalityRejection> {
    // No parameters - always legal
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// MATERIAL WORLD COMMAND GATES
// ============================================================================

pub fn validate_material_world_set_barrel_water(
    liters: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if liters < 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "liters".into(),
                message: format!("Water volume cannot be negative, got {}", liters),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_set_barrel_leak() -> Result<LegalityVerdict, LegalityRejection> {
    // Boolean toggle - always legal
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_ignite_fire() -> Result<LegalityVerdict, LegalityRejection> {
    // No parameters - always legal
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_extinguish_fire() -> Result<LegalityVerdict, LegalityRejection> {
    // No parameters - always legal
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_set_wetness(
    wetness_percent: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if !(0.0..=100.0).contains(&wetness_percent) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "wetness_percent".into(),
                message: format!("Wetness must be 0-100, got {}", wetness_percent),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_material_world_update(
    delta_time: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if delta_time <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "delta_time".into(),
                message: format!("Delta time must be positive, got {}", delta_time),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// NAV/DOOR/INVENTORY COMMAND GATES
// ============================================================================

pub fn validate_nav_door_set_blocked(reason: &str) -> Result<LegalityVerdict, LegalityRejection> {
    if reason.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "reason".into(),
                message: "Door blocked reason cannot be empty".into(),
            },
        });
    }
    if reason.len() > 256 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "reason".into(),
                message: format!("Door blocked reason too long: {} > 256", reason.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_nav_set_path(start: [f32; 3], destination: [f32; 3]) -> Result<LegalityVerdict, LegalityRejection> {
    // Paths always have valid coordinates - no specific validation needed
    let _ = (start, destination);
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_add_item(item_id: u32, item_name: &str, item_type: &str) -> Result<LegalityVerdict, LegalityRejection> {
    if item_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "item_id".into(),
                message: "Item ID cannot be zero".into(),
            },
        });
    }
    if item_name.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "item_name".into(),
                message: "Item name cannot be empty".into(),
            },
        });
    }
    if item_type.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "item_type".into(),
                message: "Item type cannot be empty".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_remove_item(item_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if item_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "item_id".into(),
                message: "Item ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_transfer(item_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if item_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "item_id".into(),
                message: "Item ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_equip_weapon(item_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if item_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "item_id".into(),
                message: "Item ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_world_load_state(state_json: &str) -> Result<LegalityVerdict, LegalityRejection> {
    if state_json.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "state_json".into(),
                message: "State JSON cannot be empty".into(),
            },
        });
    }
    if state_json.len() > 10_000_000 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "state_json".into(),
                message: format!("State JSON too large: {} > 10MB", state_json.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_region_request(region_key: (i32, i32, i32)) -> Result<LegalityVerdict, LegalityRejection> {
    // Region keys are always valid integer coordinates
    let _ = region_key;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_region_complete_load(
    region_key: (i32, i32, i32),
    size_bytes: usize,
) -> Result<LegalityVerdict, LegalityRejection> {
    if size_bytes == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "size_bytes".into(),
                message: "Region size cannot be zero".into(),
            },
        });
    }
    let _ = region_key;
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// POPULATION/NPC COMMAND GATES
// ============================================================================

pub fn validate_npc_create_profile(
    npc_id: u32,
    name: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if npc_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "npc_id".into(),
                message: "NPC ID cannot be zero".into(),
            },
        });
    }
    if name.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "name".into(),
                message: "NPC name cannot be empty".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_trait_value(
    field: &str,
    value: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if !(0.0..=1.0).contains(&value) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: field.into(),
                message: format!("{} must be 0-1, got {}", field, value),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_set_need(need_type: &str, value: f32) -> Result<LegalityVerdict, LegalityRejection> {
    if need_type.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "need_type".into(),
                message: "Need type cannot be empty".into(),
            },
        });
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "value".into(),
                message: format!("Need value must be 0-1, got {}", value),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_set_activity(
    activity: &str,
    duration: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if activity.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "activity".into(),
                message: "Activity cannot be empty".into(),
            },
        });
    }
    if duration <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "duration".into(),
                message: format!("Activity duration must be positive, got {}", duration),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scarcity_increase(scarcity_factor: f32) -> Result<LegalityVerdict, LegalityRejection> {
    if scarcity_factor < 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "scarcity_factor".into(),
                message: format!("Scarcity factor cannot be negative, got {}", scarcity_factor),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_set_faction(
    faction_id: u32,
    reputation: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if faction_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "faction_id".into(),
                message: "Faction ID cannot be zero".into(),
            },
        });
    }
    if !(0.0..=1.0).contains(&reputation) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "reputation".into(),
                message: format!("Reputation must be 0-1, got {}", reputation),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// TACTICS COMMAND GATES
// ============================================================================

pub fn validate_tactics_create_squad(
    squad_id: u32,
    member_ids: &[u32],
    roles: &[String],
) -> Result<LegalityVerdict, LegalityRejection> {
    if squad_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "squad_id".into(),
                message: "Squad ID cannot be zero".into(),
            },
        });
    }
    if member_ids.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "member_ids".into(),
                message: "Squad must have at least one member".into(),
            },
        });
    }
    if member_ids.len() != roles.len() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "roles".into(),
                message: format!(
                    "Member count ({}) must match role count ({})",
                    member_ids.len(),
                    roles.len()
                ),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_set_cover(npc_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if npc_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "npc_id".into(),
                message: "NPC ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_invalidate_cover() -> Result<LegalityVerdict, LegalityRejection> {
    // Position-based invalidation - always valid
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_check_cover_valid() -> Result<LegalityVerdict, LegalityRejection> {
    // Position-based check - always valid
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// ECOLOGY COMMAND GATES
// ============================================================================

pub fn validate_ecology_create_creature(
    creature_id: u32,
    species: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if creature_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "creature_id".into(),
                message: "Creature ID cannot be zero".into(),
            },
        });
    }
    if species.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "species".into(),
                message: "Species cannot be empty".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_ecology_set_creature_state(value: f32, field: &str) -> Result<LegalityVerdict, LegalityRejection> {
    if !(0.0..=1.0).contains(&value) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: field.into(),
                message: format!("{} must be 0-1, got {}", field, value),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// REASON CHAIN COMMAND GATES
// ============================================================================

pub fn validate_reason_chain_inspect_npc(npc_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if npc_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "npc_id".into(),
                message: "NPC ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// VERTICAL SLICE COMMAND GATES
// ============================================================================

pub fn validate_vertical_slice_bootstrap_scene() -> Result<LegalityVerdict, LegalityRejection> {
    // No parameters to validate - scene bootstrap is always legal
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_reset_scene() -> Result<LegalityVerdict, LegalityRejection> {
    // No parameters to validate - scene reset is always legal
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_fire_test_shot(
    weapon_entity_id: u32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if weapon_entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "weapon_entity_id".into(),
                message: "Weapon entity ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_assign_material(
    entity_id: u32,
    stack_id: u16,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if stack_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "stack_id".into(),
                message: "Stack ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_select_entity(
    entity_id: u32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

// ============================================================================
// ANIMATION COMMAND GATES
// ============================================================================

pub fn validate_animation_play_clip(
    entity_id: u32,
    clip_id: u32,
    blend_time: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if clip_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "clip_id".into(),
                message: "Clip ID cannot be zero".into(),
            },
        });
    }
    if blend_time < 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "blend_time".into(),
                message: format!("Blend time cannot be negative, got {}", blend_time),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_stop_clip(entity_id: u32) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_set_speed(
    entity_id: u32,
    speed: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if speed <= 0.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "speed".into(),
                message: format!("Animation speed must be positive, got {}", speed),
            },
        });
    }
    if speed > 100.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "speed".into(),
                message: format!("Animation speed excessively high: {} > 100", speed),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_set_weight(
    entity_id: u32,
    layer: u8,
    weight: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if !(0.0..=1.0).contains(&weight) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "weight".into(),
                message: format!("Blend weight must be 0-1, got {}", weight),
            },
        });
    }
    let _ = layer; // Layer index is a u8, always valid
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_set_ik_target(
    entity_id: u32,
    ik_chain_id: u16,
    position: [f32; 3],
    rotation: [f32; 4],
    weight: f32,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if ik_chain_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "ik_chain_id".into(),
                message: "IK chain ID cannot be zero".into(),
            },
        });
    }
    if !(0.0..=1.0).contains(&weight) {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "weight".into(),
                message: format!("IK weight must be 0-1, got {}", weight),
            },
        });
    }
    // Position and rotation are free-form; validate quaternion is approximately normalized
    let rot_magnitude_sq = rotation[0] * rotation[0]
        + rotation[1] * rotation[1]
        + rotation[2] * rotation[2]
        + rotation[3] * rotation[3];
    if rot_magnitude_sq < 0.0001 || rot_magnitude_sq > 10.0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "rotation".into(),
                message: "Rotation quaternion has invalid magnitude".into(),
            },
        });
    }
    let _ = position;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_load_clip(
    clip_id: u32,
    clip_data: &[u8],
) -> Result<LegalityVerdict, LegalityRejection> {
    if clip_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "clip_id".into(),
                message: "Clip ID cannot be zero".into(),
            },
        });
    }
    if clip_data.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "clip_data".into(),
                message: "Clip data cannot be empty".into(),
            },
        });
    }
    if clip_data.len() > 100_000_000 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "clip_data".into(),
                message: format!("Clip data too large: {} > 100MB", clip_data.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_create_state_machine(
    entity_id: u32,
    states: &[(u16, String, u32)],
    transitions: &[(u16, u16, String, f32)],
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if states.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "states".into(),
                message: "State machine must have at least one state".into(),
            },
        });
    }
    if states.len() > 1000 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "states".into(),
                message: format!("State machine has too many states: {} > 1000", states.len()),
            },
        });
    }
    for (i, (_, label, _)) in states.iter().enumerate() {
        if label.is_empty() {
            return Err(LegalityRejection {
                verdict: LegalityVerdict::Illegal,
                reason: LegalityRejectionReason::InvalidInput {
                    field: "state_label".into(),
                    message: format!("State label at index {} cannot be empty", i),
                },
            });
        }
    }
    for (i, (from, to, trigger, blend)) in transitions.iter().enumerate() {
        if *from == 0 || *to == 0 {
            return Err(LegalityRejection {
                verdict: LegalityVerdict::Illegal,
                reason: LegalityRejectionReason::InvalidInput {
                    field: "transition_states".into(),
                    message: format!(
                        "Transition at index {} has invalid state ID (from={}, to={})",
                        i, from, to
                    ),
                },
            });
        }
        if trigger.is_empty() {
            return Err(LegalityRejection {
                verdict: LegalityVerdict::Illegal,
                reason: LegalityRejectionReason::InvalidInput {
                    field: "transition_trigger".into(),
                    message: format!(
                        "Transition trigger at index {} cannot be empty",
                        i
                    ),
                },
            });
        }
        if *blend < 0.0 {
            return Err(LegalityRejection {
                verdict: LegalityVerdict::Illegal,
                reason: LegalityRejectionReason::InvalidInput {
                    field: "blend_duration".into(),
                    message: format!(
                        "Transition blend duration at index {} cannot be negative, got {}",
                        i, blend
                    ),
                },
            });
        }
    }
    let _ = (states, transitions);
    Ok(LegalityVerdict::Legal)
}

pub fn validate_animation_trigger_event(
    entity_id: u32,
    event_name: &str,
) -> Result<LegalityVerdict, LegalityRejection> {
    if entity_id == 0 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "entity_id".into(),
                message: "Entity ID cannot be zero".into(),
            },
        });
    }
    if event_name.is_empty() {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "event_name".into(),
                message: "Event name cannot be empty".into(),
            },
        });
    }
    if event_name.len() > 256 {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::InvalidInput {
                field: "event_name".into(),
                message: format!("Event name too long: {} > 256", event_name.len()),
            },
        });
    }
    Ok(LegalityVerdict::Legal)
}
