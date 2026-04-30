// Tests for l6.0-authority-core: L60AuthorityCoreMarker and CANONICAL_LEVEL

use stratumx_tooling_l6_0_authority_core::{L60AuthorityCoreMarker, CANONICAL_LEVEL};

// ============================================================================
// CANONICAL_LEVEL constant tests
// ============================================================================

#[test]
fn canonical_level_has_expected_value() {
    assert_eq!(CANONICAL_LEVEL, "l6.0-authority-core");
}

#[test]
fn canonical_level_is_not_empty() {
    assert!(!CANONICAL_LEVEL.is_empty());
}

#[test]
fn canonical_level_starts_with_l6() {
    assert!(CANONICAL_LEVEL.starts_with("l6"));
}

#[test]
fn canonical_level_contains_authority() {
    assert!(CANONICAL_LEVEL.contains("authority"));
}

#[test]
fn canonical_level_is_static_str() {
    let _: &'static str = CANONICAL_LEVEL;
}

// ============================================================================
// L60AuthorityCoreMarker tests
// ============================================================================

#[test]
fn marker_default_constructs() {
    let marker = L60AuthorityCoreMarker;
    assert_eq!(marker, L60AuthorityCoreMarker);
}

#[test]
fn marker_equality() {
    let a = L60AuthorityCoreMarker;
    let b = L60AuthorityCoreMarker;
    assert_eq!(a, b);
}

#[test]
fn marker_clone() {
    let a = L60AuthorityCoreMarker;
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn marker_copy() {
    let a = L60AuthorityCoreMarker;
    let _b = a;
    let _c = a; // a is still usable after copy
}

#[test]
fn marker_debug() {
    let marker = L60AuthorityCoreMarker;
    let debug_str = format!("{:?}", marker);
    assert!(debug_str.contains("L60AuthorityCoreMarker"));
}

#[test]
fn marker_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let a = L60AuthorityCoreMarker;
    let b = L60AuthorityCoreMarker;

    let mut hasher_a = DefaultHasher::new();
    a.hash(&mut hasher_a);
    let hash_a = hasher_a.finish();

    let mut hasher_b = DefaultHasher::new();
    b.hash(&mut hasher_b);
    let hash_b = hasher_b.finish();

    assert_eq!(hash_a, hash_b);
}

#[test]
fn marker_in_hashset() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(L60AuthorityCoreMarker);
    set.insert(L60AuthorityCoreMarker);
    assert_eq!(set.len(), 1);
}

#[test]
fn marker_in_hashmap_key() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(L60AuthorityCoreMarker, "authority-core");
    assert_eq!(map.get(&L60AuthorityCoreMarker), Some(&"authority-core"));
}

#[test]
fn marker_is_unit_struct() {
    assert_eq!(std::mem::size_of::<L60AuthorityCoreMarker>(), 0);
}

#[test]
fn marker_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<L60AuthorityCoreMarker>();
}

#[test]
fn marker_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<L60AuthorityCoreMarker>();
}

#[test]
fn marker_implements_debug() {
    fn assert_debug<T: std::fmt::Debug>() {}
    assert_debug::<L60AuthorityCoreMarker>();
}

#[test]
fn marker_implements_hash() {
    use std::hash::Hash;
    fn assert_hash<T: Hash>() {}
    assert_hash::<L60AuthorityCoreMarker>();
}

#[test]
fn marker_implements_eq() {
    fn assert_eq<T: Eq>() {}
    assert_eq::<L60AuthorityCoreMarker>();
}

#[test]
fn module_exports_canonical_level() {
    let level = stratumx_tooling_l6_0_authority_core::CANONICAL_LEVEL;
    assert_eq!(level, "l6.0-authority-core");
}

#[test]
fn module_exports_marker() {
    let _marker = stratumx_tooling_l6_0_authority_core::L60AuthorityCoreMarker;
}
