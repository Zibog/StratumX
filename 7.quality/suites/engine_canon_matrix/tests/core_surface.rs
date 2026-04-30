use engine_core::*;

// === Generation Tests ===

#[test]
fn test_generation_creation() {
    let gen = Generation(42);
    assert_eq!(gen, Generation(42));
}

#[test]
fn test_generation_ordering() {
    assert!(Generation(0) < Generation(1));
    assert!(Generation(100) > Generation(99));
}

#[test]
fn test_generation_equality() {
    assert_eq!(Generation(5), Generation(5));
    assert_ne!(Generation(5), Generation(6));
}

// === ComponentTypeId Tests ===

#[test]
fn test_component_type_id_creation() {
    let id = ComponentTypeId(100);
    assert_eq!(id, ComponentTypeId(100));
}

#[test]
fn test_component_type_id_equality() {
    let a = ComponentTypeId(1);
    let b = ComponentTypeId(1);
    let c = ComponentTypeId(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_component_type_id_ordering() {
    assert!(ComponentTypeId(10) < ComponentTypeId(20));
}

#[test]
fn test_component_type_id_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(ComponentTypeId(1));
    set.insert(ComponentTypeId(2));
    set.insert(ComponentTypeId(1)); // duplicate
    assert_eq!(set.len(), 2);
}

// === EngineCoreError Tests ===

#[test]
fn test_engine_core_error_variants() {
    let invalid = EngineCoreError::InvalidDescriptor("test");
    assert!(matches!(invalid, EngineCoreError::InvalidDescriptor(_)));
}

#[test]
fn test_engine_core_error_display() {
    let error = EngineCoreError::InvalidDescriptor("test message");
    let msg = format!("{}", error);
    assert!(msg.contains("test message"));
}

// === EngineCoreResult Tests ===

#[test]
fn test_engine_core_result_ok() {
    let result: EngineCoreResult<i32> = Ok(42);
    assert!(result.is_ok());
    assert!(matches!(result, Ok(42)));
}

#[test]
fn test_engine_core_result_err() {
    let result: EngineCoreResult<i32> = Err(EngineCoreError::InvalidDescriptor("fail"));
    assert!(result.is_err());
}
