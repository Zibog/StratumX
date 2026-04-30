#[test]
fn test_traversal_entry_bind_matching_locality() {
    let mut descriptor = make_descriptor(AccessMode::READ);
    descriptor.locality = LocalityClass::Spatial;
    let result = traversal_entry_bind(&descriptor, false, false, LocalityClass::Spatial);
    assert!(result.is_ok());
}

// === AccessDescriptor Tests ===

#[test]
fn test_access_descriptor_creation() {
    let desc = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(99),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: false,
    };
    assert_eq!(desc.plan_id.0, 99);
    assert_eq!(desc.locality, LocalityClass::Cache);
    assert_eq!(desc.scratch, ScratchClass::Owned);
}

#[test]
fn test_access_descriptor_equality() {
    let a = make_descriptor(AccessMode::READ);
    let b = make_descriptor(AccessMode::READ);
    assert_eq!(a, b);
}

// === Integration: Access Control Flow ===

#[test]
fn test_full_access_flow_read_then_write() {
    let handle = make_handle();

    // Read phase
    let read_desc = make_descriptor(AccessMode::READ);
    let read_view = make_read_view(read_desc, handle).unwrap();
    assert_eq!(read_view.descriptor.mode, AccessMode::READ);

    // Write phase
    let write_desc = make_descriptor(AccessMode::WRITE);
    let write_window = make_write_window(write_desc, handle).unwrap();
    assert!(write_window.descriptor.mode.contains(AccessMode::WRITE));
}
