#[test]
fn residency_control_counts_case_0() {
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
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_1() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 2,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_2() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 3,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_3() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 4,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_4() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 5,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_5() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 6,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_6() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 7,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_7() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 8,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_8() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 9,
        residency_set: ResidencySet::Hot,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
#[test]
fn residency_control_counts_case_9() {
    let memory = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    let mut s = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    s.pin(ResidencyDescriptor {
        asset_key: 10,
        residency_set: ResidencySet::StreamingResident,
    });
    let m = s.metrics(&memory);
    assert!(m.resident_items + m.streaming_items >= 1);
}
