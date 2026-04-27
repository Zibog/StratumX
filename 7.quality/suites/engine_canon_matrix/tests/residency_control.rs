// Residency Control Service Tests

use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};
use engine_residency_control::{
    ResidencyConfig, ResidencyControlService, ResidencyDescriptor, ResidencySet,
};
use engine_storage_layout::LayoutClass;

#[test]
fn test_residency_control_pressure() {
    let mut memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    memory
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 60,
            layout_class: Some(LayoutClass::Sparse),
        })
        .expect("reserve");
    let mut residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 0,
        streaming_item_budget: 0,
    });
    residency.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    let metrics = residency.metrics(&memory);
    assert_eq!(metrics.pressure, PressureClass::Critical);
}
