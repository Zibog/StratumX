use engine_memory_control::{MemoryConfig, MemoryControlService, PressureClass};
use engine_residency_control::{
    ResidencyConfig, ResidencyControlService, ResidencyDescriptor, ResidencySet,
};

#[test]
fn residency_becomes_critical_when_hot_budget_is_exceeded() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut service = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 1,
        streaming_item_budget: 1,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 2,
        residency_set: ResidencySet::Hot,
    });
    assert_eq!(service.metrics(&memory).pressure, PressureClass::Critical);
}

#[test]
fn unpin_reduces_resident_counts() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut service = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 4,
        streaming_item_budget: 4,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    service.pin(ResidencyDescriptor {
        asset_key: 2,
        residency_set: ResidencySet::StreamingResident,
    });
    service.unpin(1);
    let metrics = service.metrics(&memory);
    assert_eq!(metrics.resident_items, 0);
    assert_eq!(metrics.streaming_items, 1);
}
