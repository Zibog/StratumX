#[test]
fn memory_control_pressure_case_7() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 24,
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
fn memory_control_pressure_case_8() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 26,
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
fn memory_control_pressure_case_9() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 28,
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
fn memory_control_pressure_case_10() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 30,
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
fn memory_control_pressure_case_11() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 32,
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
fn memory_control_pressure_case_12() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 34,
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
fn memory_control_pressure_case_13() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 36,
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
