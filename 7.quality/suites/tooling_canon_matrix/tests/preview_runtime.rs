// Tests for l6.12-preview-runtime: CANONICAL_LEVEL, marker, and public API

use stratumx_tooling_l6_12_preview_runtime::*;

// ============================================================================
// CANONICAL_LEVEL constant tests
// ============================================================================

#[test]
fn canonical_level_value() {
    assert_eq!(CANONICAL_LEVEL, "l6.12-preview-runtime");
}

#[test]
fn canonical_level_not_empty() {
    assert!(!CANONICAL_LEVEL.is_empty());
}

#[test]
fn canonical_level_starts_with_l6() {
    assert!(CANONICAL_LEVEL.starts_with("l6"));
}

#[test]
fn canonical_level_contains_preview() {
    assert!(CANONICAL_LEVEL.contains("preview"));
}

// ============================================================================
// L612PreviewRuntimeMarker tests
// ============================================================================

#[test]
fn marker_default() {
    let m = L612PreviewRuntimeMarker::default();
    assert_eq!(m, L612PreviewRuntimeMarker);
}

#[test]
fn marker_equality() {
    assert_eq!(L612PreviewRuntimeMarker, L612PreviewRuntimeMarker);
}

#[test]
fn marker_copy() {
    let a = L612PreviewRuntimeMarker;
    let _b = a;
    let _c = a;
}

#[test]
fn marker_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    L612PreviewRuntimeMarker.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    L612PreviewRuntimeMarker.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn marker_in_hashset() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(L612PreviewRuntimeMarker);
    set.insert(L612PreviewRuntimeMarker);
    assert_eq!(set.len(), 1);
}

#[test]
fn marker_in_hashmap() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(L612PreviewRuntimeMarker, "preview-runtime");
    assert_eq!(map.get(&L612PreviewRuntimeMarker), Some(&"preview-runtime"));
}

#[test]
fn marker_is_unit_struct() {
    assert_eq!(std::mem::size_of::<L612PreviewRuntimeMarker>(), 0);
}

#[test]
fn marker_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<L612PreviewRuntimeMarker>();
}

#[test]
fn marker_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<L612PreviewRuntimeMarker>();
}

#[test]
fn marker_implements_debug() {
    fn assert_debug<T: std::fmt::Debug>() {}
    assert_debug::<L612PreviewRuntimeMarker>();
}

#[test]
fn marker_debug_contains_name() {
    let debug = format!("{:?}", L612PreviewRuntimeMarker);
    assert!(debug.contains("L612PreviewRuntimeMarker"));
}

// ============================================================================
// VerticalSliceSession - basic API tests (requires engine init)
// ============================================================================

#[test]
fn vertical_slice_session_type_exists() {
    // Verify the type is accessible
    fn _assert_type<T>() {}
    _assert_type::<VerticalSliceSession>();
}

#[test]
fn vertical_slice_session_is_not_default_safe() {
    // VerticalSliceSession::default() calls new() which calls engine init
    // and may panic. We verify the type exists but don't instantiate.
    let _type_name = std::any::type_name::<VerticalSliceSession>();
    assert!(_type_name.contains("VerticalSliceSession"));
}

// ============================================================================
// EditorAuthoringSession tests
// ============================================================================

#[test]
fn editor_authoring_session_type_exists() {
    fn _assert_type<T>() {}
    _assert_type::<EditorAuthoringSession>();
}

#[test]
fn editor_authoring_session_is_default() {
    let session = EditorAuthoringSession::default();
    assert!(!session.has_runtime_session());
}

#[test]
fn editor_authoring_session_new() {
    let session = EditorAuthoringSession::new();
    assert!(!session.has_runtime_session());
}

#[test]
fn editor_authoring_session_world_summary_without_runtime() {
    let session = EditorAuthoringSession::new();
    let summary = session.get_world_summary();
    assert!(summary.active_scene.is_none());
    assert_eq!(summary.entity_count, 0);
    assert!(summary.active_actor.is_none());
}

#[test]
fn editor_authoring_session_type_name() {
    let _type_name = std::any::type_name::<EditorAuthoringSession>();
    assert!(_type_name.contains("EditorAuthoringSession"));
}

// ============================================================================
// Module structure tests
// ============================================================================

#[test]
fn crate_exports_canonical_level() {
    let level = stratumx_tooling_l6_12_preview_runtime::CANONICAL_LEVEL;
    assert_eq!(level, "l6.12-preview-runtime");
}

#[test]
fn crate_exports_marker() {
    let _marker = stratumx_tooling_l6_12_preview_runtime::L612PreviewRuntimeMarker;
}

#[test]
fn crate_exports_vertical_slice_session() {
    fn _assert_type<T>() {}
    _assert_type::<stratumx_tooling_l6_12_preview_runtime::VerticalSliceSession>();
}

#[test]
fn crate_exports_editor_authoring_session() {
    fn _assert_type<T>() {}
    _assert_type::<stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession>();
}

// ============================================================================
// initialize_vertical_slice_in_executor function tests
// ============================================================================

#[test]
fn initialize_function_exists() {
    // Just verify the function signature is accessible
    let _fn = initialize_vertical_slice_in_executor as fn(
        &mut stratumx_tooling_l6_0_tool_session::CommandExecutor,
    ) -> Result<(), String>;
}
