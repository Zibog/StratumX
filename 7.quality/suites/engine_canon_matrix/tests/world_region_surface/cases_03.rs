#[test]
fn test_region_state_is_simulation_active() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    let desc = substrate.region_descriptor(region).unwrap();
    assert!(!desc.is_simulation_active());

    substrate.activate_region(region, RegionPriority(10));
    let desc = substrate.region_descriptor(region).unwrap();
    assert!(desc.is_simulation_active());
}
