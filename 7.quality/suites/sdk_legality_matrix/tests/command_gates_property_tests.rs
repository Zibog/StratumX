//! Property-based tests for SDK command legality gates
//!
//! Feature: stratumx-100-percent-canon-coverage
//! Property 11: SDK Legality Gate Coverage
//!
//! **Validates: Requirements 6.5**
//!
//! This test verifies that all operations defined in SDK ingress packets have
//! corresponding legality gate validation functions that can accept or reject
//! the operation with descriptive error messages.

use legality_gates::*;
use sdk_compat::LegalityVerdict;

// ============================================================================
// Property 11: SDK Legality Gate Coverage
// ============================================================================

/// Property 11: For any operation defined in SDK ingress packets, there must be
/// a corresponding legality gate validation function that can accept or reject
/// the operation.
///
/// This test verifies that all command validation functions exist and can
/// properly accept valid inputs and reject invalid inputs with descriptive errors.

#[test]
fn property_11_scene_commands_have_legality_gates() {
    // Scene: CreateEmpty
    assert!(validate_scene_create_empty("valid_scene").is_ok());
    assert!(validate_scene_create_empty("").is_err());
    
    // Scene: CreateEntityFromAsset
    assert!(validate_scene_create_entity_from_asset(1, "entity").is_ok());
    assert!(validate_scene_create_entity_from_asset(0, "entity").is_err());
    assert!(validate_scene_create_entity_from_asset(1, "").is_err());
    
    // Scene: SetTransform
    assert!(validate_scene_set_transform(1).is_ok());
    assert!(validate_scene_set_transform(0).is_err());
    
    // Scene: DeleteEntity
    assert!(validate_scene_delete_entity(1).is_ok());
    assert!(validate_scene_delete_entity(0).is_err());
    
    // Scene: GetEntityDetails
    assert!(validate_scene_get_entity_details(1).is_ok());
    assert!(validate_scene_get_entity_details(0).is_err());
}

#[test]
fn property_11_material_commands_have_legality_gates() {
    // Material: CreateArchetype
    assert!(validate_material_create_archetype("steel", 5.0, 7850.0).is_ok());
    assert!(validate_material_create_archetype("", 5.0, 7850.0).is_err());
    assert!(validate_material_create_archetype("steel", -1.0, 7850.0).is_err());
    assert!(validate_material_create_archetype("steel", 11.0, 7850.0).is_err());
    assert!(validate_material_create_archetype("steel", 5.0, -1.0).is_err());
    
    // Material: CreateStack
    assert!(validate_material_create_stack("stack1").is_ok());
    assert!(validate_material_create_stack("").is_err());
    
    // Material: StackAddLayer
    assert!(validate_material_stack_add_layer(1, 10.0).is_ok());
    assert!(validate_material_stack_add_layer(0, 10.0).is_err());
    assert!(validate_material_stack_add_layer(1, 0.0).is_err());
    assert!(validate_material_stack_add_layer(1, -1.0).is_err());
    
    // Material: AssignStack
    assert!(validate_material_assign_stack(1, 1).is_ok());
    assert!(validate_material_assign_stack(0, 1).is_err());
    assert!(validate_material_assign_stack(1, 0).is_err());
}

#[test]
fn property_11_terrain_commands_have_legality_gates() {
    // Terrain: CreatePatch
    assert!(validate_terrain_create_patch([100.0, 100.0], "patch1").is_ok());
    assert!(validate_terrain_create_patch([0.0, 100.0], "patch1").is_err());
    assert!(validate_terrain_create_patch([100.0, 0.0], "patch1").is_err());
    assert!(validate_terrain_create_patch([100.0, 100.0], "").is_err());
    
    // Terrain: PaintSurface
    assert!(validate_terrain_paint_surface(1, 5.0, 1).is_ok());
    assert!(validate_terrain_paint_surface(0, 5.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, 0.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, -1.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, 5.0, 0).is_err());
}

#[test]
fn property_11_actor_commands_have_legality_gates() {
    // Actor: SpawnPreset
    assert!(validate_actor_spawn_preset(1).is_ok());
    assert!(validate_actor_spawn_preset(0).is_err());
    
    // Actor: AttachWeapon
    assert!(validate_actor_attach_weapon(1, 1).is_ok());
    assert!(validate_actor_attach_weapon(0, 1).is_err());
    assert!(validate_actor_attach_weapon(1, 0).is_err());
    
    // Actor: SetActive
    assert!(validate_actor_set_active(1).is_ok());
    assert!(validate_actor_set_active(0).is_err());
}

#[test]
fn property_11_asset_commands_have_legality_gates() {
    // Asset: Import
    assert!(validate_asset_import("path/to/asset.obj", "asset1").is_ok());
    assert!(validate_asset_import("", "asset1").is_err());
    assert!(validate_asset_import("path/to/asset.obj", "").is_err());
    
    // Asset: GetDetails
    assert!(validate_asset_get_details(1).is_ok());
    assert!(validate_asset_get_details(0).is_err());
}

#[test]
fn property_11_sky_weather_commands_have_legality_gates() {
    // Sky: SetTimeOfDay
    assert!(validate_sky_set_time_of_day(12.0).is_ok());
    assert!(validate_sky_set_time_of_day(0.0).is_ok());
    assert!(validate_sky_set_time_of_day(23.99).is_ok());
    assert!(validate_sky_set_time_of_day(-1.0).is_err());
    assert!(validate_sky_set_time_of_day(24.0).is_err());
    
    // Sky: SetDayOfYear
    assert!(validate_sky_set_day_of_year(1).is_ok());
    assert!(validate_sky_set_day_of_year(365).is_ok());
    assert!(validate_sky_set_day_of_year(0).is_err());
    assert!(validate_sky_set_day_of_year(366).is_err());
    
    // Sky: SetLatitude
    assert!(validate_sky_set_latitude(0.0).is_ok());
    assert!(validate_sky_set_latitude(90.0).is_ok());
    assert!(validate_sky_set_latitude(-90.0).is_ok());
    assert!(validate_sky_set_latitude(91.0).is_err());
    assert!(validate_sky_set_latitude(-91.0).is_err());
    
    // Sky: SetNormalizedValue
    assert!(validate_sky_set_normalized_value("cloud_coverage", 0.5).is_ok());
    assert!(validate_sky_set_normalized_value("cloud_coverage", 0.0).is_ok());
    assert!(validate_sky_set_normalized_value("cloud_coverage", 1.0).is_ok());
    assert!(validate_sky_set_normalized_value("cloud_coverage", -0.1).is_err());
    assert!(validate_sky_set_normalized_value("cloud_coverage", 1.1).is_err());
    
    // Sky: SetRain
    assert!(validate_sky_set_rain(10.0).is_ok());
    assert!(validate_sky_set_rain(0.0).is_ok());
    assert!(validate_sky_set_rain(-1.0).is_err());
    
    // Sky: StepSimulation
    assert!(validate_sky_step_simulation(1.0).is_ok());
    assert!(validate_sky_step_simulation(0.1).is_ok());
    assert!(validate_sky_step_simulation(3600.0).is_ok());
    assert!(validate_sky_step_simulation(0.0).is_err());
    assert!(validate_sky_step_simulation(-1.0).is_err());
    assert!(validate_sky_step_simulation(3601.0).is_err());
    
    // Storm: Create
    assert!(validate_storm_create(10.0, 0.5, 20.0).is_ok());
    assert!(validate_storm_create(0.0, 0.5, 20.0).is_err());
    assert!(validate_storm_create(-1.0, 0.5, 20.0).is_err());
    assert!(validate_storm_create(10.0, -0.1, 20.0).is_err());
    assert!(validate_storm_create(10.0, 1.1, 20.0).is_err());
    assert!(validate_storm_create(10.0, 0.5, -1.0).is_err());
    
    // Storm: Update
    assert!(validate_storm_update(1).is_ok());
    assert!(validate_storm_update(0).is_err());
}

#[test]
fn property_11_ballistics_commands_have_legality_gates() {
    // Ballistics: FireActiveActor
    assert!(validate_ballistics_fire_active_actor().is_ok());
}

#[test]
fn property_11_destruction_commands_have_legality_gates() {
    // Destruction: TriggerBlast
    assert!(validate_destruction_trigger_blast(1000.0).is_ok());
    assert!(validate_destruction_trigger_blast(0.0).is_err());
    assert!(validate_destruction_trigger_blast(-1.0).is_err());
    
    // Destruction: SetIntegrity
    assert!(validate_destruction_set_integrity(0.5).is_ok());
    assert!(validate_destruction_set_integrity(0.0).is_ok());
    assert!(validate_destruction_set_integrity(1.0).is_ok());
    assert!(validate_destruction_set_integrity(-0.1).is_err());
    assert!(validate_destruction_set_integrity(1.1).is_err());
}

#[test]
fn property_11_material_world_commands_have_legality_gates() {
    // MaterialWorld: SetBarrelWater
    assert!(validate_material_world_set_barrel_water(10.0).is_ok());
    assert!(validate_material_world_set_barrel_water(0.0).is_ok());
    assert!(validate_material_world_set_barrel_water(-1.0).is_err());
    
    // MaterialWorld: SetWetness
    assert!(validate_material_world_set_wetness(50.0).is_ok());
    assert!(validate_material_world_set_wetness(0.0).is_ok());
    assert!(validate_material_world_set_wetness(100.0).is_ok());
    assert!(validate_material_world_set_wetness(-1.0).is_err());
    assert!(validate_material_world_set_wetness(101.0).is_err());
    
    // MaterialWorld: Update
    assert!(validate_material_world_update(0.016).is_ok());
    assert!(validate_material_world_update(0.0).is_err());
    assert!(validate_material_world_update(-1.0).is_err());
}

#[test]
fn property_11_population_npc_commands_have_legality_gates() {
    // Population: CreateNpcProfile
    assert!(validate_npc_create_profile(1, "John").is_ok());
    assert!(validate_npc_create_profile(0, "John").is_err());
    assert!(validate_npc_create_profile(1, "").is_err());
    
    // Population: TraitValue
    assert!(validate_npc_trait_value("aggression", 0.5).is_ok());
    assert!(validate_npc_trait_value("aggression", 0.0).is_ok());
    assert!(validate_npc_trait_value("aggression", 1.0).is_ok());
    assert!(validate_npc_trait_value("aggression", -0.1).is_err());
    assert!(validate_npc_trait_value("aggression", 1.1).is_err());
}

#[test]
fn property_11_vertical_slice_commands_have_legality_gates() {
    // VerticalSlice: FireTestShot
    assert!(validate_vertical_slice_fire_test_shot(1).is_ok());
    assert!(validate_vertical_slice_fire_test_shot(0).is_err());
    
    // VerticalSlice: AssignMaterial
    assert!(validate_vertical_slice_assign_material(1, 1).is_ok());
    assert!(validate_vertical_slice_assign_material(0, 1).is_err());
    assert!(validate_vertical_slice_assign_material(1, 0).is_err());
    
    // VerticalSlice: SelectEntity
    assert!(validate_vertical_slice_select_entity(1).is_ok());
    assert!(validate_vertical_slice_select_entity(0).is_err());
}

#[test]
fn property_11_all_rejections_have_descriptive_messages() {
    // Verify that all rejection errors contain descriptive messages
    
    // Scene command rejection
    let result = validate_scene_create_empty("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.verdict, LegalityVerdict::Illegal);
    match err.reason {
        sdk_compat::LegalityRejectionReason::InvalidInput { field, message } => {
            assert_eq!(field, "scene_name");
            assert!(message.contains("cannot be empty"));
        }
        _ => panic!("Expected InvalidInput rejection reason"),
    }
    
    // Material command rejection
    let result = validate_material_create_archetype("steel", 11.0, 7850.0);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err.reason {
        sdk_compat::LegalityRejectionReason::InvalidInput { field, message } => {
            assert_eq!(field, "hardness_mohs");
            assert!(message.contains("0-10"));
        }
        _ => panic!("Expected InvalidInput rejection reason"),
    }
    
    // Sky command rejection
    let result = validate_sky_set_time_of_day(25.0);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err.reason {
        sdk_compat::LegalityRejectionReason::InvalidInput { field, message } => {
            assert_eq!(field, "hours");
            assert!(message.contains("0-24"));
        }
        _ => panic!("Expected InvalidInput rejection reason"),
    }
}

#[test]
fn property_11_boundary_value_testing() {
    // Test boundary values for all numeric validations
    
    // Hardness: 0-10 range
    assert!(validate_material_create_archetype("mat", 0.0, 1000.0).is_ok());
    assert!(validate_material_create_archetype("mat", 10.0, 1000.0).is_ok());
    assert!(validate_material_create_archetype("mat", -0.001, 1000.0).is_err());
    assert!(validate_material_create_archetype("mat", 10.001, 1000.0).is_err());
    
    // Time of day: 0-24 range
    assert!(validate_sky_set_time_of_day(0.0).is_ok());
    assert!(validate_sky_set_time_of_day(23.999).is_ok());
    assert!(validate_sky_set_time_of_day(-0.001).is_err());
    assert!(validate_sky_set_time_of_day(24.0).is_err());
    
    // Day of year: 1-365 range
    assert!(validate_sky_set_day_of_year(1).is_ok());
    assert!(validate_sky_set_day_of_year(365).is_ok());
    assert!(validate_sky_set_day_of_year(0).is_err());
    assert!(validate_sky_set_day_of_year(366).is_err());
    
    // Latitude: -90 to 90 range
    assert!(validate_sky_set_latitude(-90.0).is_ok());
    assert!(validate_sky_set_latitude(90.0).is_ok());
    assert!(validate_sky_set_latitude(-90.001).is_err());
    assert!(validate_sky_set_latitude(90.001).is_err());
    
    // Normalized values: 0-1 range
    assert!(validate_sky_set_normalized_value("test", 0.0).is_ok());
    assert!(validate_sky_set_normalized_value("test", 1.0).is_ok());
    assert!(validate_sky_set_normalized_value("test", -0.001).is_err());
    assert!(validate_sky_set_normalized_value("test", 1.001).is_err());
}

#[test]
fn property_11_string_length_validation() {
    // Test string length limits
    
    // Scene name: max 256 characters
    let valid_name = "a".repeat(256);
    assert!(validate_scene_create_empty(&valid_name).is_ok());
    
    let too_long_name = "a".repeat(257);
    assert!(validate_scene_create_empty(&too_long_name).is_err());
    
    // Entity label: max 256 characters
    let valid_label = "b".repeat(256);
    assert!(validate_scene_create_entity_from_asset(1, &valid_label).is_ok());
    
    let too_long_label = "b".repeat(257);
    assert!(validate_scene_create_entity_from_asset(1, &too_long_label).is_err());
}

#[test]
fn property_11_positive_value_validation() {
    // Test that all values requiring positive numbers reject zero and negative
    
    // Density must be positive
    assert!(validate_material_create_archetype("mat", 5.0, 0.001).is_ok());
    assert!(validate_material_create_archetype("mat", 5.0, 0.0).is_err());
    assert!(validate_material_create_archetype("mat", 5.0, -1.0).is_err());
    
    // Thickness must be positive
    assert!(validate_material_stack_add_layer(1, 0.001).is_ok());
    assert!(validate_material_stack_add_layer(1, 0.0).is_err());
    assert!(validate_material_stack_add_layer(1, -1.0).is_err());
    
    // Terrain size must be positive
    assert!(validate_terrain_create_patch([0.001, 0.001], "patch").is_ok());
    assert!(validate_terrain_create_patch([0.0, 1.0], "patch").is_err());
    assert!(validate_terrain_create_patch([1.0, 0.0], "patch").is_err());
    assert!(validate_terrain_create_patch([-1.0, 1.0], "patch").is_err());
    
    // Paint radius must be positive
    assert!(validate_terrain_paint_surface(1, 0.001, 1).is_ok());
    assert!(validate_terrain_paint_surface(1, 0.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, -1.0, 1).is_err());
    
    // Blast energy must be positive
    assert!(validate_destruction_trigger_blast(0.001).is_ok());
    assert!(validate_destruction_trigger_blast(0.0).is_err());
    assert!(validate_destruction_trigger_blast(-1.0).is_err());
    
    // Delta time must be positive
    assert!(validate_material_world_update(0.001).is_ok());
    assert!(validate_material_world_update(0.0).is_err());
    assert!(validate_material_world_update(-1.0).is_err());
    
    // Simulation step must be positive
    assert!(validate_sky_step_simulation(0.001).is_ok());
    assert!(validate_sky_step_simulation(0.0).is_err());
    assert!(validate_sky_step_simulation(-1.0).is_err());
    
    // Storm radius must be positive
    assert!(validate_storm_create(0.001, 0.5, 10.0).is_ok());
    assert!(validate_storm_create(0.0, 0.5, 10.0).is_err());
    assert!(validate_storm_create(-1.0, 0.5, 10.0).is_err());
}

#[test]
fn property_11_id_validation() {
    // Test that all ID fields reject zero (reserved for invalid/null)
    
    assert!(validate_scene_set_transform(1).is_ok());
    assert!(validate_scene_set_transform(0).is_err());
    
    assert!(validate_scene_delete_entity(1).is_ok());
    assert!(validate_scene_delete_entity(0).is_err());
    
    assert!(validate_material_stack_add_layer(1, 1.0).is_ok());
    assert!(validate_material_stack_add_layer(0, 1.0).is_err());
    
    assert!(validate_terrain_paint_surface(1, 1.0, 1).is_ok());
    assert!(validate_terrain_paint_surface(0, 1.0, 1).is_err());
    assert!(validate_terrain_paint_surface(1, 1.0, 0).is_err());
    
    assert!(validate_actor_spawn_preset(1).is_ok());
    assert!(validate_actor_spawn_preset(0).is_err());
    
    assert!(validate_actor_attach_weapon(1, 1).is_ok());
    assert!(validate_actor_attach_weapon(0, 1).is_err());
    assert!(validate_actor_attach_weapon(1, 0).is_err());
    
    assert!(validate_asset_get_details(1).is_ok());
    assert!(validate_asset_get_details(0).is_err());
    
    assert!(validate_storm_update(1).is_ok());
    assert!(validate_storm_update(0).is_err());
    
    assert!(validate_npc_create_profile(1, "name").is_ok());
    assert!(validate_npc_create_profile(0, "name").is_err());
}
