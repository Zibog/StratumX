use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};
use engine_residency_control::{
    ResidencyConfig, ResidencyControlService, ResidencyDescriptor, ResidencySet,
};

#[test]
fn residency_streaming_budget_overflow_is_critical() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut service = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 4,
        streaming_item_budget: 1,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::StreamingResident,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 2,
        residency_set: ResidencySet::StreamingResident,
    });
    assert_eq!(service.metrics(&memory).pressure, PressureClass::Critical);
}

#[test]
fn residency_passes_through_memory_pressure_when_budgets_are_legal() {
    let mut memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    memory
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 80,
            layout_class: None,
        })
        .unwrap();
    memory
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 60,
            layout_class: None,
        })
        .unwrap();
    let mut service = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 7,
        residency_set: ResidencySet::Hot,
    });
    assert_eq!(service.metrics(&memory).pressure, PressureClass::Elevated);
}
