//! Editor and authoring command legality gates.
//!
//! This module contains scene, actor, inventory, population, tactics, ecology,
//! and vertical-slice validations that remain editor-driven in the legacy SDK
//! command surface.

use super::common::{
    invalid, max_len, non_empty, non_negative, non_zero_u16, non_zero_u32, normalized, positive,
    GateResult,
};
use super::verdict::LegalityVerdict;

pub fn validate_actor_spawn_preset(preset_id: u16) -> GateResult {
    non_zero_u16(preset_id, "preset_id", "Preset ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_actor_attach_weapon(actor_id: u32, weapon_profile_id: u16) -> GateResult {
    non_zero_u32(actor_id, "actor_id", "Actor ID")?;
    non_zero_u16(weapon_profile_id, "weapon_profile_id", "Weapon profile ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_actor_set_active(actor_id: u32) -> GateResult {
    non_zero_u32(actor_id, "actor_id", "Actor ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_create_empty(scene_name: &str) -> GateResult {
    non_empty(scene_name, "scene_name", "Scene name cannot be empty")?;
    max_len(scene_name, 256, "scene_name", "Scene name")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_create_entity_from_asset(asset_id: u32, label: &str) -> GateResult {
    non_zero_u32(asset_id, "asset_id", "Asset ID")?;
    non_empty(label, "label", "Entity label cannot be empty")?;
    max_len(label, 256, "label", "Entity label")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_set_transform(entity_id: u32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_delete_entity(entity_id: u32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scene_get_entity_details(entity_id: u32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_ballistics_fire_active_actor() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_nav_door_set_blocked(reason: &str) -> GateResult {
    non_empty(reason, "reason", "Door blocked reason cannot be empty")?;
    max_len(reason, 256, "reason", "Door blocked reason")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_nav_set_path(start: [f32; 3], destination: [f32; 3]) -> GateResult {
    let _ = (start, destination);
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_add_item(item_id: u32, item_name: &str, item_type: &str) -> GateResult {
    non_zero_u32(item_id, "item_id", "Item ID")?;
    non_empty(item_name, "item_name", "Item name cannot be empty")?;
    non_empty(item_type, "item_type", "Item type cannot be empty")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_remove_item(item_id: u32) -> GateResult {
    non_zero_u32(item_id, "item_id", "Item ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_transfer(item_id: u32) -> GateResult {
    non_zero_u32(item_id, "item_id", "Item ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_inventory_equip_weapon(item_id: u32) -> GateResult {
    non_zero_u32(item_id, "item_id", "Item ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_create_profile(npc_id: u32, name: &str) -> GateResult {
    non_zero_u32(npc_id, "npc_id", "NPC ID")?;
    non_empty(name, "name", "NPC name cannot be empty")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_trait_value(field: &str, value: f32) -> GateResult {
    normalized(value, field, field)?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_set_need(need_type: &str, value: f32) -> GateResult {
    non_empty(need_type, "need_type", "Need type cannot be empty")?;
    normalized(value, "value", "Need value")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_set_activity(activity: &str, duration: f32) -> GateResult {
    non_empty(activity, "activity", "Activity cannot be empty")?;
    positive(duration, "duration", "Activity duration")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_scarcity_increase(scarcity_factor: f32) -> GateResult {
    non_negative(scarcity_factor, "scarcity_factor", "Scarcity factor")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_npc_set_faction(faction_id: u32, reputation: f32) -> GateResult {
    non_zero_u32(faction_id, "faction_id", "Faction ID")?;
    normalized(reputation, "reputation", "Reputation")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_create_squad(
    squad_id: u32,
    member_ids: &[u32],
    roles: &[String],
) -> GateResult {
    non_zero_u32(squad_id, "squad_id", "Squad ID")?;
    if member_ids.is_empty() {
        return Err(invalid("member_ids", "Squad must have at least one member"));
    }
    if member_ids.len() != roles.len() {
        return Err(invalid(
            "roles",
            format!(
                "Member count ({}) must match role count ({})",
                member_ids.len(),
                roles.len()
            ),
        ));
    }
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_set_cover(npc_id: u32) -> GateResult {
    non_zero_u32(npc_id, "npc_id", "NPC ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_invalidate_cover() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_tactics_check_cover_valid() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_ecology_create_creature(creature_id: u32, species: &str) -> GateResult {
    non_zero_u32(creature_id, "creature_id", "Creature ID")?;
    non_empty(species, "species", "Species cannot be empty")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_ecology_set_creature_state(value: f32, field: &str) -> GateResult {
    normalized(value, field, field)?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_bootstrap_scene() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_reset_scene() -> GateResult {
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_fire_test_shot(weapon_entity_id: u32) -> GateResult {
    non_zero_u32(weapon_entity_id, "weapon_entity_id", "Weapon entity ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_assign_material(entity_id: u32, stack_id: u16) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    non_zero_u16(stack_id, "stack_id", "Stack ID")?;
    Ok(LegalityVerdict::Legal)
}

pub fn validate_vertical_slice_select_entity(entity_id: u32) -> GateResult {
    non_zero_u32(entity_id, "entity_id", "Entity ID")?;
    Ok(LegalityVerdict::Legal)
}
