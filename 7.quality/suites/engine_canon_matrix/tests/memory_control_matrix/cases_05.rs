#[test]
fn memory_control_pressure_case_28() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 66,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_29() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 68,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
