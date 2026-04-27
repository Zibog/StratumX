use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};

#[test]
fn memory_control_tracks_pressure() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 80,
            layout_class: None,
        })
        .unwrap();
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 60,
            layout_class: None,
        })
        .unwrap();
    assert_eq!(service.metrics().pressure, PressureClass::Elevated);
}
