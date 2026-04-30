#[test]
fn wetness_threshold_prevents_ignition_deterministically() {
    let material = CombustibleMaterial::Wood;
    let wetness_threshold = material.wetness_ignition_threshold();

    // Just below threshold should allow ignition
    let mut obj1 = CombustibleObject::new([0.0, 0.0, 0.0], material);
    obj1.wetness.wetness_percent = wetness_threshold - 1.0;
    assert!(obj1.can_ignite());

    // At threshold should block ignition
    let mut obj2 = CombustibleObject::new([0.0, 0.0, 0.0], material);
    obj2.wetness.wetness_percent = wetness_threshold;
    assert!(!obj2.can_ignite());

    // Above threshold should block ignition
    let mut obj3 = CombustibleObject::new([0.0, 0.0, 0.0], material);
    obj3.wetness.wetness_percent = wetness_threshold + 10.0;
    assert!(!obj3.can_ignite());
}

#[test]
fn rain_extinguishes_burning_material() {
    let mut obj = CombustibleObject::new([0.0, 0.0, 0.0], CombustibleMaterial::Wood);

    // Ignite material
    let ignited = obj.apply_heat(350.0, 0.0);
    assert!(ignited);
    assert!(obj.fire.burning);

    // Apply heavy rain
    obj.apply_rain(60.0, 1.0);

    // Fire should be extinguished
    assert!(!obj.fire.burning);
    assert!(obj.wetness.wetness_percent > 50.0);
}

#[test]
fn fuel_exhaustion_extinguishes_fire() {
    let mut fire = FireState::new();
    fire.burning = true;
    fire.fuel_remaining_percent = 1.0;

    // Update until fuel exhausted
    fire.update(2.0);

    // Fire should be extinguished
    assert!(!fire.burning);
    assert_eq!(fire.fuel_remaining_percent, 0.0);
    assert_eq!(fire.temperature_celsius, 20.0);
}

#[test]
fn drying_rate_increases_with_temperature() {
    let material = CombustibleMaterial::Wood;

    let rate_20c = material.drying_rate_percent_per_sec(20.0);
    let rate_100c = material.drying_rate_percent_per_sec(100.0);
    let rate_300c = material.drying_rate_percent_per_sec(300.0);

    assert!(rate_100c > rate_20c);
    assert!(rate_300c > rate_100c);
}
