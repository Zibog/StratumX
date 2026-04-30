#[test]
fn test_access_mode_read() {
    assert!(AccessMode::READ.contains(AccessMode::READ));
    assert!(!AccessMode::READ.contains(AccessMode::WRITE));
}

#[test]
fn test_access_mode_write() {
    assert!(AccessMode::WRITE.contains(AccessMode::WRITE));
}

#[test]
fn test_access_mode_staged() {
    assert!(AccessMode::STAGED.contains(AccessMode::STAGED));
}

#[test]
fn test_access_mode_mixed() {
    let mixed = AccessMode::MIXED;
    assert!(mixed.contains(AccessMode::READ));
    assert!(mixed.contains(AccessMode::STAGED));
    assert!(!mixed.contains(AccessMode::WRITE));
}

#[test]
fn test_access_mode_bits() {
    assert_eq!(AccessMode::READ.bits(), 0b0001);
    assert_eq!(AccessMode::WRITE.bits(), 0b0010);
    assert_eq!(AccessMode::STAGED.bits(), 0b0100);
    assert_eq!(AccessMode::MIXED.bits(), 0b0101);
}

#[test]
fn test_access_mode_from_bits() {
    let mode = AccessMode::from_bits(0b0011);
    assert!(mode.is_some());
    let mode = mode.unwrap();
    assert!(mode.contains(AccessMode::READ));
    assert!(mode.contains(AccessMode::WRITE));
}

#[test]
fn test_access_mode_from_bits_invalid() {
    assert!(AccessMode::from_bits(0b1000).is_none());
}

// === TraversalPlanId Tests ===

#[test]
fn test_plan_id_creation() {
    let id = TraversalPlanId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_plan_id_equality() {
    assert_eq!(TraversalPlanId(1), TraversalPlanId(1));
    assert_ne!(TraversalPlanId(1), TraversalPlanId(2));
}

// === ScratchClass Tests ===

#[test]
fn test_scratch_class_variants() {
    let borrowed = ScratchClass::Borrowed;
    let owned = ScratchClass::Owned;
    assert_ne!(borrowed, owned);
}

// === ReadView Tests ===

#[test]
fn test_make_read_view_success() {
    let descriptor = make_descriptor(AccessMode::READ);
    let handle = make_handle();
    let result = make_read_view(descriptor, handle);
    assert!(result.is_ok());
}

#[test]
fn test_make_read_view_fails_without_read_mode() {
    let descriptor = make_descriptor(AccessMode::WRITE);
    let handle = make_handle();
    let result = make_read_view(descriptor, handle);
    assert!(result.is_err());
}

#[test]
fn test_make_read_view_with_mixed_mode() {
    let descriptor = make_descriptor(AccessMode::MIXED);
    let handle = make_handle();
    let result = make_read_view(descriptor, handle);
    assert!(result.is_ok());
}

#[test]
fn test_read_view_preserves_descriptor() {
    let descriptor = make_descriptor(AccessMode::READ);
    let handle = make_handle();
    let view = make_read_view(descriptor, handle).unwrap();
    assert_eq!(view.descriptor.mode, AccessMode::READ);
    assert_eq!(view.anchor, handle);
}

// === WriteWindow Tests ===

#[test]
fn test_make_write_window_with_write_mode() {
    let descriptor = make_descriptor(AccessMode::WRITE);
    let handle = make_handle();
    let result = make_write_window(descriptor, handle);
    assert!(result.is_ok());
}

#[test]
fn test_make_write_window_with_staged_mode() {
    let mut descriptor = make_descriptor(AccessMode::STAGED);
    descriptor.staged_mutation_handoff = true;
    let handle = make_handle();
    let result = make_write_window(descriptor, handle);
    assert!(result.is_ok());
}

#[test]
fn test_make_write_window_fails_with_read_only() {
    let descriptor = make_descriptor(AccessMode::READ);
    let handle = make_handle();
    let result = make_write_window(descriptor, handle);
    assert!(result.is_err());
}

#[test]
fn test_make_write_window_fails_without_staged_handoff() {
    let mut descriptor = make_descriptor(AccessMode::WRITE);
    descriptor.staged_mutation_handoff = false;
    let handle = make_handle();
    let result = make_write_window(descriptor, handle);
    assert!(result.is_err());
}

#[test]
fn test_write_window_preserves_descriptor() {
    let descriptor = make_descriptor(AccessMode::WRITE);
    let handle = make_handle();
    let window = make_write_window(descriptor, handle).unwrap();
    assert_eq!(window.descriptor.mode, AccessMode::WRITE);
    assert!(window.descriptor.staged_mutation_handoff);
}

// === Traversal Entry Bind Tests ===

#[test]
fn test_traversal_entry_bind_cache_hit_no_recompile() {
    let descriptor = make_descriptor(AccessMode::READ);
    let result = traversal_entry_bind(&descriptor, true, false, LocalityClass::Cache);
    assert!(result.is_ok());
}

#[test]
fn test_traversal_entry_bind_cache_miss_with_recompile() {
    let descriptor = make_descriptor(AccessMode::READ);
    let result = traversal_entry_bind(&descriptor, false, true, LocalityClass::Cache);
    assert!(result.is_ok());
}

#[test]
fn test_traversal_entry_bind_fails_cache_hit_with_recompile() {
    let descriptor = make_descriptor(AccessMode::READ);
    let result = traversal_entry_bind(&descriptor, true, true, LocalityClass::Cache);
    assert!(result.is_err());
}

#[test]
fn test_traversal_entry_bind_fails_locality_mismatch() {
    let descriptor = make_descriptor(AccessMode::READ);
    let result = traversal_entry_bind(&descriptor, false, false, LocalityClass::Spatial);
    assert!(result.is_err());
}

