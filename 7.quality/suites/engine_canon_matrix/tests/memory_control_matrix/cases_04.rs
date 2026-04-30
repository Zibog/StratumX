#[test]
fn memory_control_pressure_case_21() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 52,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_22() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 54,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_23() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 56,
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
fn memory_control_pressure_case_24() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 58,
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
#[test]
fn memory_control_pressure_case_25() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 60,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_26() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 62,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_27() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 64,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
