#![allow(unused_imports)]
use super::*;
use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};

#[test]
fn pin_hot_increases_resident_count() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    assert_eq!(s.metrics(&memory).resident_items, 1);
}
#[test]
fn pin_streaming_increases_streaming_count() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::StreamingResident,
    });
    assert_eq!(s.metrics(&memory).streaming_items, 1);
}
#[test]
fn unpin_removes_descriptor() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    s.unpin(1);
    assert_eq!(s.metrics(&memory).resident_items, 0);
}
#[test]
fn budget_overflow_yields_critical_pressure() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 0,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    assert_eq!(s.metrics(&memory).pressure, PressureClass::Critical);
}
#[test]
fn legal_residency_passes_through_memory_pressure() {
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
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 1,
        residency_set: ResidencySet::Hot,
    });
    assert_eq!(s.metrics(&memory).pressure, PressureClass::Elevated);
}
