use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};

#[test]
fn memory_control_escalates_to_critical_at_ninety_percent() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 100,
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 80,
            layout_class: None,
        })
        .unwrap();
    assert_eq!(service.metrics().pressure, PressureClass::Critical);
}

#[test]
fn releasing_memory_lowers_pressure_class() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 90,
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 50,
            layout_class: None,
        })
        .unwrap();
    assert_eq!(service.metrics().pressure, PressureClass::Elevated);
    service.release_heap(60);
    service.release_staging(50);
    assert_eq!(service.metrics().pressure, PressureClass::Healthy);
}

#[test]
fn heap_and_staging_budget_overflow_are_rejected() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 10,
        staging_budget_bytes: 10,
    });
    assert!(service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 11,
            layout_class: None
        })
        .is_err());
    assert!(service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 11,
            layout_class: None
        })
        .is_err());
}
