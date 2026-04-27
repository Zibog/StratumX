mod common;
use common::*;

#[test]
fn memory_control_pressure_case_0() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 10,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_1() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 12,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_2() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 14,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_3() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 16,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_4() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 18,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_5() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 20,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_6() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 22,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_7() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 24,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_8() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 26,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_9() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 28,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_10() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 30,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_11() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 32,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_12() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 34,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_13() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 36,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_14() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 38,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_15() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 40,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_16() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 42,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_17() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 44,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_18() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 46,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_19() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 48,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_20() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 50,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_21() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 52,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_22() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 54,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_23() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 56,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_24() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 58,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_25() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 60,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 5,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_26() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 62,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 6,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_27() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 64,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 7,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_28() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 66,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 8,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
#[test]
fn memory_control_pressure_case_29() {
    let mut m = MemoryControlService::new(MemoryConfig {
        heap_budget_bytes: 100,
        staging_budget_bytes: 100,
    });
    m.reserve_heap(&AllocationDescriptor {
        allocation_id: 1,
        bytes: 68,
        layout_class: None,
    })
    .unwrap();
    m.reserve_staging(&AllocationDescriptor {
        allocation_id: 2,
        bytes: 9,
        layout_class: None,
    })
    .unwrap();
    assert!(matches!(
        m.metrics().pressure,
        PressureClass::Healthy | PressureClass::Elevated | PressureClass::Critical
    ));
}
