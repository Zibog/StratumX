#[test]
fn residency_control_counts_case_20() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 21,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_21() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 22,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_22() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 23,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_23() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 24,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_24() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 25,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
