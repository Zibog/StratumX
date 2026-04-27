use engine_memory_control::{AllocationDescriptor, MemoryConfig, MemoryControlService};
use engine_residency_control::{
    ResidencyConfig, ResidencyControlService, ResidencyDescriptor, ResidencySet,
};

#[test]
fn residency_metrics_follow_memory_pressure() {
    let mut memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    memory
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 95,
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
    assert_eq!(service.metrics(&memory).resident_items, 1);
}
