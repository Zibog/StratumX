// Material Burn and Fire Law Tests

use engine_core::EngineCoreError;
use engine_material::{
    BurnAftermathPolicy, BurnResponseFamily, CombustibleMaterial, FireExposureContext,
    MaterialStateModifier, PersistentBurnResult, ResponseFamilyGroup,
};
use engine_world::MaterialWorldExecutor;

#[test]
fn fire_exposure_runtime_emits_receipt_and_material_event() {
    let mut world = MaterialWorldExecutor::new(45.0);
    let object = world.create_combustible_object([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    let receipt = world
        .apply_fire_exposure_to_object(
            object,
            FireExposureContext {
                flame_temperature_celsius: 400.0,
                exposure_duration_seconds: 1.5,
                oxygen_availability: 1.0,
            },
            0.0,
        )
        .unwrap();

    let event = world.latest_material_event().expect("event emitted");
    assert!(receipt.ignited);
    assert_eq!(receipt.burn_family, BurnResponseFamily::SlowBurn);
    assert_eq!(
        receipt.aftermath_policy,
        BurnAftermathPolicy::ConvertToCharred
    );
    assert_eq!(receipt.persistent_result, PersistentBurnResult::Charred);
    assert_eq!(event, &receipt.to_material_consequence_event());
    assert_eq!(
        event.response_family_group,
        ResponseFamilyGroup::PhysicalThermal
    );
    assert!(event.persistence_required);
    assert!(event
        .state_modifiers
        .contains(&MaterialStateModifier::Charred));
}

#[test]
fn wetness_changes_ignition_branch_without_bypassing_runtime_path() {
    let mut world = MaterialWorldExecutor::new(45.0);
    let object = world.create_combustible_object([0.0, 0.0, 0.0], CombustibleMaterial::Wood);
    world
        .combustible_objects
        .get_mut(object)
        .expect("object exists")
        .wetness
        .add_water(80.0, 0.0);

    let receipt = world
        .apply_fire_exposure_to_object(
            object,
            FireExposureContext {
                flame_temperature_celsius: 350.0,
                exposure_duration_seconds: 0.25,
                oxygen_availability: 1.0,
            },
            0.0,
        )
        .unwrap();

    let event = world.latest_material_event().expect("event emitted");
    assert!(!receipt.ignited);
    assert_eq!(receipt.aftermath_policy, BurnAftermathPolicy::RemainIntact);
    assert_eq!(receipt.persistent_result, PersistentBurnResult::Intact);
    assert!(event.state_modifiers.contains(&MaterialStateModifier::Wet));
}

#[test]
fn aftermath_policy_changes_persistent_result() {
    let mut world = MaterialWorldExecutor::new(45.0);
    let wood = world.create_combustible_object([0.0, 0.0, 0.0], CombustibleMaterial::Wood);
    let plastic = world.create_combustible_object([1.0, 0.0, 0.0], CombustibleMaterial::Plastic);

    let wood_receipt = world
        .apply_fire_exposure_to_object(
            wood,
            FireExposureContext {
                flame_temperature_celsius: 400.0,
                exposure_duration_seconds: 2.0,
                oxygen_availability: 1.0,
            },
            0.0,
        )
        .unwrap();
    let plastic_receipt = world
        .apply_fire_exposure_to_object(
            plastic,
            FireExposureContext {
                flame_temperature_celsius: 500.0,
                exposure_duration_seconds: 2.0,
                oxygen_availability: 1.0,
            },
            0.0,
        )
        .unwrap();

    assert_eq!(
        wood_receipt.persistent_result,
        PersistentBurnResult::Charred
    );
    assert_eq!(
        plastic_receipt.persistent_result,
        PersistentBurnResult::Removed
    );
    assert_ne!(
        wood_receipt.persistent_result,
        plastic_receipt.persistent_result
    );
}

#[test]
fn invalid_fire_exposure_profile_is_rejected() {
    let mut world = MaterialWorldExecutor::new(45.0);
    let object = world.create_combustible_object([0.0, 0.0, 0.0], CombustibleMaterial::Grass);

    let err = world
        .apply_fire_exposure_to_object(
            object,
            FireExposureContext {
                flame_temperature_celsius: 300.0,
                exposure_duration_seconds: 0.0,
                oxygen_availability: 1.0,
            },
            0.0,
        )
        .unwrap_err();
    assert_eq!(
        err,
        EngineCoreError::InvalidDescriptor("fire exposure duration must be positive")
    );
}

#[test]
fn same_fire_exposure_input_produces_same_digest() {
    let context = FireExposureContext {
        flame_temperature_celsius: 360.0,
        exposure_duration_seconds: 1.0,
        oxygen_availability: 0.9,
    };
    let mut left = MaterialWorldExecutor::new(45.0);
    let mut right = MaterialWorldExecutor::new(45.0);
    let left_id = left.create_combustible_object([0.0, 0.0, 0.0], CombustibleMaterial::Paper);
    let right_id = right.create_combustible_object([0.0, 0.0, 0.0], CombustibleMaterial::Paper);

    let left_receipt = left
        .apply_fire_exposure_to_object(left_id, context.clone(), 0.0)
        .unwrap();
    let right_receipt = right
        .apply_fire_exposure_to_object(right_id, context, 0.0)
        .unwrap();

    assert_eq!(
        left_receipt.deterministic_digest,
        right_receipt.deterministic_digest
    );
    assert_eq!(left_receipt, right_receipt);
}
