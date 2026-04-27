// Memory Control Service Tests

use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};
use engine_storage_layout::LayoutClass;

#[test]
fn test_memory_control_reserve() {
    let mut service = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    service
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 60,
            layout_class: Some(LayoutClass::Sparse),
        })
        .expect("heap reserve");
    service
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 2,
            bytes: 20,
            layout_class: None,
        })
        .expect("staging reserve");
    let metrics = service.metrics();
    assert_eq!(metrics.heap_bytes, 60);
    assert_eq!(metrics.staging_bytes, 20);
    assert_eq!(metrics.pressure, PressureClass::Healthy);
}
