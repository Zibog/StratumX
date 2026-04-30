#[test]
fn memory_control_pressure_case_0() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 10,
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
fn memory_control_pressure_case_1() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 12,
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
fn memory_control_pressure_case_2() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 14,
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
fn memory_control_pressure_case_3() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 16,
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
fn memory_control_pressure_case_4() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 18,
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
fn memory_control_pressure_case_5() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 20,
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
fn memory_control_pressure_case_6() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 22,
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
