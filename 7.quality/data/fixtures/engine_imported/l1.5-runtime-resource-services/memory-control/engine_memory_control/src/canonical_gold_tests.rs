#![allow(unused_imports)]
use super::*;

#[test]
fn heap_reservation_within_budget_is_ok() {
    let mut s = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    assert!(s
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 50,
            layout_class: None
        })
        .is_ok());
}
#[test]
fn heap_reservation_over_budget_fails() {
    let mut s = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    assert!(s
        .reserve_heap(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 101,
            layout_class: None
        })
        .is_err());
}
#[test]
fn staging_reservation_over_budget_fails() {
    let mut s = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    assert!(s
        .reserve_staging(&AllocationDescriptor {
            allocation_id: 1,
            bytes: 101,
            layout_class: None
        })
        .is_err());
}
#[test]
fn release_is_saturating() {
    let mut s = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    s.release_heap(999);
    assert_eq!(s.metrics().heap_bytes, 0);
}
#[test]
fn pressure_thresholds_are_inclusive() {
    let mut s = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    s.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 70,
        layout_class: None,
    })
    .unwrap();
    s.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 70,
        layout_class: None,
    })
    .unwrap();
    assert_eq!(s.metrics().pressure, PressureClass::Elevated);
}
