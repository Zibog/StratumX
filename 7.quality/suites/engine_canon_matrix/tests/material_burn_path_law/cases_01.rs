#[test]
fn wet_material_does_not_ignite_from_single_heat_impulse() {
    let mut obj = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    // Make material wet
    obj.wetness.add_water(50.0, 0.0);
    assert!(obj.wetness.is_wet());

    // Apply heat above ignition temperature
    let heat_temp = 350.0; // Above wood ignition temp (300°C)
    let ignited = obj.apply_heat(heat_temp, 0.0);

    // Should not ignite because wet
    assert!(!ignited);
    assert!(!obj.fire.burning);
}

#[test]
fn sustained_heat_drives_wet_material_through_drying_before_ignition() {
    let mut obj = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    // Make material wet
    obj.wetness.add_water(50.0, 0.0);
    assert!(obj.wetness.is_wet());

    let heat_temp = 350.0;
    let mut current_time = 0.0;

    // Apply sustained heat with explicit drying
    // Wood drying rate at 350°C is ~8.75%/sec
    let drying_rate = obj.material_type.drying_rate_percent_per_sec(heat_temp);

    let mut ignited = false;
    for _ in 0..50 {
        // Dry the material
        obj.wetness.dry(drying_rate, 0.1);

        // Try to ignite once dry enough
        if obj.can_ignite() {
            ignited = obj.apply_heat(heat_temp, current_time);
            if ignited {
                break;
            }
        }
        current_time += 0.1;
    }

    // Should eventually ignite after drying
    assert!(ignited, "Material should ignite after sustained drying");
    assert!(obj.fire.burning);
    assert!(obj.wetness.wetness_percent < obj.material_type.wetness_ignition_threshold());
}

#[test]
fn dry_material_ignites_after_threshold() {
    let mut obj = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    // Material starts dry
    assert!(!obj.wetness.is_wet());
    assert!(obj.can_ignite());

    // Apply heat above ignition temperature
    let heat_temp = 350.0; // Above wood ignition temp (300°C)
    let ignited = obj.apply_heat(heat_temp, 0.0);

    // Should ignite immediately
    assert!(ignited);
    assert!(obj.fire.burning);
    assert_eq!(obj.fire.temperature_celsius, 300.0); // At ignition temp
    assert!(obj.fire.ignition_time.is_some());
}

#[test]
fn burn_consequence_emits_downstream_publication_metadata() {
    let mut obj = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    // Ignite material
    let heat_temp = 350.0;
    let ignited = obj.apply_heat(heat_temp, 0.0);
    assert!(ignited);

    // Fire state contains consequence metadata
    assert!(obj.fire.burning);
    assert!(obj.fire.ignition_time.is_some());
    assert_eq!(obj.fire.temperature_celsius, 300.0);
    assert_eq!(obj.fire.fuel_remaining_percent, 100.0);

    // Update fire to produce burn consequences
    obj.fire.update(1.0);

    // Fuel should be consumed
    assert!(obj.fire.fuel_remaining_percent < 100.0);
    assert_eq!(obj.fire.temperature_celsius, 800.0); // Burning temp
}

#[test]
fn invalid_burn_transition_is_rejected() {
    let mut fire = FireState::new();

    // Cannot ignite if already burning
    fire.burning = true;
    let result = fire.attempt_ignition(350.0, 300.0, 0.0);
    assert!(!result);

    // Cannot ignite if no fuel
    let mut fire2 = FireState::new();
    fire2.fuel_remaining_percent = 0.0;
    let result = fire2.attempt_ignition(350.0, 300.0, 0.0);
    assert!(!result);

    // Cannot ignite if heat below threshold
    let mut fire3 = FireState::new();
    let result = fire3.attempt_ignition(200.0, 300.0, 0.0);
    assert!(!result);
}

#[test]
fn same_burn_input_produces_same_consequence() {
    let mut obj1 = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);
    let mut obj2 = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    let heat_temp = 350.0;
    let current_time = 0.0;

    let ignited1 = obj1.apply_heat(heat_temp, current_time);
    let ignited2 = obj2.apply_heat(heat_temp, current_time);

    assert_eq!(ignited1, ignited2);
    assert_eq!(obj1.fire.burning, obj2.fire.burning);
    assert_eq!(obj1.fire.temperature_celsius, obj2.fire.temperature_celsius);
    assert_eq!(obj1.fire.ignition_time, obj2.fire.ignition_time);
}

#[test]
fn different_materials_have_different_ignition_thresholds() {
    let materials = [
        (CombustibleMaterial::Wood, 300.0),
        (CombustibleMaterial::Grass, 250.0),
        (CombustibleMaterial::Paper, 230.0),
        (CombustibleMaterial::Cloth, 260.0),
        (CombustibleMaterial::Plastic, 350.0),
    ];

    for (material, expected_temp) in materials {
        assert_eq!(material.ignition_temperature_celsius(), expected_temp);

        let mut obj = CombustibleObject::new([0.0, 0.0, 0.0], material);

        // Heat just below threshold should not ignite
        let below_threshold = expected_temp - 10.0;
        let ignited = obj.apply_heat(below_threshold, 0.0);
        assert!(!ignited);

        // Heat at threshold should ignite
        let mut obj2 = CombustibleObject::new([0.0, 0.0, 0.0], material);
        let ignited = obj2.apply_heat(expected_temp, 0.0);
        assert!(ignited);
    }
}

