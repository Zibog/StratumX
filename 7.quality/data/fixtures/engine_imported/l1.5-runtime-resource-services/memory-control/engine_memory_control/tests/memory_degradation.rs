use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};

#[test]
fn pressure_thresholds_are_inclusive() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 70,
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 70,
            layout_class: None,
        })
        .unwrap();
    assert_eq!(service.metrics().pressure, PressureClass::Elevated);

    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 3,
            bytes: 20,
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 4,
            bytes: 20,
            layout_class: None,
        })
        .unwrap();
    assert_eq!(service.metrics().pressure, PressureClass::Critical);
}

#[test]
fn releasing_more_than_reserved_saturates_to_zero() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 10,
            layout_class: None,
        })
        .unwrap();
    service.release_heap(999);
    assert_eq!(service.metrics().heap_bytes, 0);
}
