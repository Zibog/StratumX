#[test]
fn zero_allocations_fail_and_pressure_signal_publishes_degrade_bridge() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 0,
    });

    let zero = service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 0,
            layout_class: None,
        })
        .unwrap_err();
    assert_eq!(
        zero,
        EngineCoreError::InvalidDescriptor("memory allocation size must be non-zero")
    );

    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 70,
            layout_class: None,
        })
        .expect("elevated reservation succeeds");
    let elevated = service
        .check_pressure_signal()
        .expect("elevated transition should publish");
    assert_eq!(elevated.previous, PressureClass::Healthy);
    assert_eq!(elevated.current, PressureClass::Elevated);
    assert_eq!(elevated.reserved_heap_bytes, 70);
    assert!(elevated.degrade_bridge.degrade_lod);
    assert!(!elevated.degrade_bridge.degrade_texture_resolution);

    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 3,
            bytes: 25,
            layout_class: None,
        })
        .expect("critical reservation succeeds");
    let critical = service
        .check_pressure_signal()
        .expect("critical transition should publish");
    assert_eq!(critical.current, PressureClass::Critical);
    assert_eq!(critical.reserved_heap_bytes, 95);
    assert!(critical.degrade_bridge.degrade_texture_resolution);
    assert!(critical.degrade_bridge.shed_non_critical_allocations);
}
