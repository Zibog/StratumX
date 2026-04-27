// Audio Preview System Tests
//
// Tests for Task 13.4: Connect audio preview to runtime kernel through SDK
//
// **Requirements Validated:**
// - Requirement 9.6: Audio preview system SHALL connect to runtime kernel through proper SDK interfaces
//
// **Test Coverage:**
// 1. Preview connection establishment
// 2. Preview mode support (isolated and contextual)
// 3. Preview session management
// 4. Preview cleanup and disconnection

use stratumx_tooling_l6_0_tool_session::{ObjectClass, PreviewMode, ToolingRuntime};

#[test]
fn test_audio_preview_isolated_mode() {
    let mut runtime = ToolingRuntime::new();

    // Create audio source
    let source = runtime
        .create_object("test_source", ObjectClass::Asset)
        .expect("create source should succeed");

    // Request isolated preview
    let session = runtime
        .request_audio_preview(source, PreviewMode::Isolated)
        .expect("preview request should succeed");

    // Verify session properties
    assert_eq!(session.source_handle, source);
    assert_eq!(session.mode, PreviewMode::Isolated);
    assert!(session.active);
    assert!(!session.session_id.is_empty());

    // Verify preview is active
    assert!(runtime.is_preview_active());
    assert!(runtime.active_preview_session().is_some());
}

#[test]
fn test_audio_preview_contextual_mode() {
    let mut runtime = ToolingRuntime::new();

    // Create audio source
    let source = runtime
        .create_object("test_source", ObjectClass::Asset)
        .expect("create source should succeed");

    // Request contextual preview
    let session = runtime
        .request_audio_preview(source, PreviewMode::Contextual)
        .expect("preview request should succeed");

    // Verify session properties
    assert_eq!(session.source_handle, source);
    assert_eq!(session.mode, PreviewMode::Contextual);
    assert!(session.active);

    // Verify preview is active
    assert!(runtime.is_preview_active());
}

#[test]
fn test_audio_preview_stop() {
    let mut runtime = ToolingRuntime::new();

    // Create audio source
    let source = runtime
        .create_object("test_source", ObjectClass::Asset)
        .expect("create source should succeed");

    // Start preview
    runtime
        .request_audio_preview(source, PreviewMode::Isolated)
        .expect("preview request should succeed");
    assert!(runtime.is_preview_active());

    // Stop preview
    runtime
        .stop_audio_preview()
        .expect("stop preview should succeed");

    // Verify preview is stopped
    assert!(!runtime.is_preview_active());
    assert!(runtime.active_preview_session().is_none());
}

#[test]
fn test_audio_preview_replaces_active_preview() {
    let mut runtime = ToolingRuntime::new();

    // Create two audio sources
    let source1 = runtime
        .create_object("source1", ObjectClass::Asset)
        .expect("create source1 should succeed");
    let source2 = runtime
        .create_object("source2", ObjectClass::Asset)
        .expect("create source2 should succeed");

    // Start first preview
    let session1 = runtime
        .request_audio_preview(source1, PreviewMode::Isolated)
        .expect("preview request 1 should succeed");
    assert_eq!(session1.source_handle, source1);

    // Start second preview (should replace first)
    let session2 = runtime
        .request_audio_preview(source2, PreviewMode::Contextual)
        .expect("preview request 2 should succeed");
    assert_eq!(session2.source_handle, source2);
    assert_eq!(session2.mode, PreviewMode::Contextual);

    // Verify only second preview is active
    let active = runtime.active_preview_session().unwrap();
    assert_eq!(active.source_handle, source2);
    assert_eq!(active.mode, PreviewMode::Contextual);
}

#[test]
fn test_audio_preview_invalid_source() {
    let mut runtime = ToolingRuntime::new();

    // Try to preview non-existent source
    use stratumx_tooling_l6_0_tool_session::ObjectHandle;
    let invalid_source = ObjectHandle(9999);

    let result = runtime.request_audio_preview(invalid_source, PreviewMode::Isolated);

    // Should fail with error
    assert!(result.is_err());
}

#[test]
fn test_audio_preview_stop_when_no_active_preview() {
    let mut runtime = ToolingRuntime::new();

    // Stop preview when none is active (should succeed gracefully)
    let result = runtime.stop_audio_preview();
    assert!(result.is_ok());

    // Verify no preview is active
    assert!(!runtime.is_preview_active());
}

#[test]
fn test_audio_preview_session_unique_ids() {
    let mut runtime = ToolingRuntime::new();

    // Create audio source
    let source = runtime
        .create_object("test_source", ObjectClass::Asset)
        .expect("create source should succeed");

    // Request multiple previews and verify unique session IDs
    let session1 = runtime
        .request_audio_preview(source, PreviewMode::Isolated)
        .expect("preview request 1 should succeed");
    runtime.stop_audio_preview().expect("stop should succeed");

    let session2 = runtime
        .request_audio_preview(source, PreviewMode::Isolated)
        .expect("preview request 2 should succeed");

    // Session IDs should be different
    assert_ne!(session1.session_id, session2.session_id);
}

#[test]
fn test_audio_preview_mode_switching() {
    let mut runtime = ToolingRuntime::new();

    // Create audio source
    let source = runtime
        .create_object("test_source", ObjectClass::Asset)
        .expect("create source should succeed");

    // Start isolated preview
    let session1 = runtime
        .request_audio_preview(source, PreviewMode::Isolated)
        .expect("preview request should succeed");
    assert_eq!(session1.mode, PreviewMode::Isolated);

    // Switch to contextual preview (should replace)
    let session2 = runtime
        .request_audio_preview(source, PreviewMode::Contextual)
        .expect("preview request should succeed");
    assert_eq!(session2.mode, PreviewMode::Contextual);

    // Verify contextual mode is active
    let active = runtime.active_preview_session().unwrap();
    assert_eq!(active.mode, PreviewMode::Contextual);
}
