// Tests for l6.6-tool-diagnostics-events: DiagnosticEvent, DiagnosticsPublisher

use stratumx_tooling_l6_6_tool_diagnostics_events::*;

// ============================================================================
// CANONICAL_LEVEL constant tests
// ============================================================================

#[test]
fn canonical_level_value() {
    assert_eq!(CANONICAL_LEVEL, "l6.6-tool-diagnostics-events");
}

#[test]
fn canonical_level_not_empty() {
    assert!(!CANONICAL_LEVEL.is_empty());
}

// ============================================================================
// L66ToolDiagnosticsEventsMarker tests
// ============================================================================

#[test]
fn marker_default() {
    let m = L66ToolDiagnosticsEventsMarker::default();
    assert_eq!(m, L66ToolDiagnosticsEventsMarker);
}

#[test]
fn marker_equality() {
    assert_eq!(L66ToolDiagnosticsEventsMarker, L66ToolDiagnosticsEventsMarker);
}

#[test]
fn marker_copy() {
    let a = L66ToolDiagnosticsEventsMarker;
    let _b = a;
    let _c = a;
}

#[test]
fn marker_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    L66ToolDiagnosticsEventsMarker.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    L66ToolDiagnosticsEventsMarker.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn marker_in_hashset() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(L66ToolDiagnosticsEventsMarker);
    set.insert(L66ToolDiagnosticsEventsMarker);
    assert_eq!(set.len(), 1);
}

#[test]
fn marker_is_unit_struct() {
    assert_eq!(std::mem::size_of::<L66ToolDiagnosticsEventsMarker>(), 0);
}

// ============================================================================
// DiagnosticSeverity tests
// ============================================================================

#[test]
fn diagnostic_severity_variants() {
    let severities = [
        DiagnosticSeverity::Info,
        DiagnosticSeverity::Warning,
        DiagnosticSeverity::Error,
    ];
    assert_eq!(severities.len(), 3);
}

#[test]
fn diagnostic_severity_equality() {
    assert_eq!(DiagnosticSeverity::Info, DiagnosticSeverity::Info);
    assert_ne!(DiagnosticSeverity::Info, DiagnosticSeverity::Error);
}

#[test]
fn diagnostic_severity_serializes_info() {
    let json = serde_json::to_string(&DiagnosticSeverity::Info).unwrap();
    assert!(json.contains("Info"));
}

#[test]
fn diagnostic_severity_serializes_warning() {
    let json = serde_json::to_string(&DiagnosticSeverity::Warning).unwrap();
    assert!(json.contains("Warning"));
}

#[test]
fn diagnostic_severity_serializes_error() {
    let json = serde_json::to_string(&DiagnosticSeverity::Error).unwrap();
    assert!(json.contains("Error"));
}

#[test]
fn diagnostic_severity_deserializes() {
    let s: DiagnosticSeverity = serde_json::from_str("\"Warning\"").unwrap();
    assert_eq!(s, DiagnosticSeverity::Warning);
}

#[test]
fn diagnostic_severity_roundtrip() {
    for sev in [
        DiagnosticSeverity::Info,
        DiagnosticSeverity::Warning,
        DiagnosticSeverity::Error,
    ] {
        let json = serde_json::to_string(&sev).unwrap();
        let restored: DiagnosticSeverity = serde_json::from_str(&json).unwrap();
        assert_eq!(sev, restored);
    }
}

// ============================================================================
// DiagnosticEvent tests
// ============================================================================

#[test]
fn diagnostic_event_new() {
    let event = DiagnosticEvent::new(
        1,
        DiagnosticSeverity::Info,
        "test_source",
        "Test message",
    );
    assert_eq!(event.event_id, 1);
    assert_eq!(event.severity, DiagnosticSeverity::Info);
    assert_eq!(event.source, "test_source");
    assert_eq!(event.message, "Test message");
    assert!(event.command_id.is_none());
    assert!(event.timestamp > 0);
}

#[test]
fn diagnostic_event_with_command_id() {
    let event = DiagnosticEvent::new(2, DiagnosticSeverity::Error, "src", "err")
        .with_command_id(42);
    assert_eq!(event.command_id, Some(42));
}

#[test]
fn diagnostic_event_without_command_id() {
    let event = DiagnosticEvent::new(3, DiagnosticSeverity::Warning, "src", "warn");
    assert!(event.command_id.is_none());
}

#[test]
fn diagnostic_event_all_severities() {
    for severity in [
        DiagnosticSeverity::Info,
        DiagnosticSeverity::Warning,
        DiagnosticSeverity::Error,
    ] {
        let event = DiagnosticEvent::new(1, severity, "src", "msg");
        assert_eq!(event.severity, severity);
    }
}

#[test]
fn diagnostic_event_equality() {
    let e1 = DiagnosticEvent::new(1, DiagnosticSeverity::Info, "src", "msg");
    let e2 = DiagnosticEvent::new(1, DiagnosticSeverity::Info, "src", "msg");
    // Events have different timestamps (created at different times), so we compare fields
    assert_eq!(e1.event_id, e2.event_id);
    assert_eq!(e1.severity, e2.severity);
    assert_eq!(e1.source, e2.source);
    assert_eq!(e1.message, e2.message);
}

#[test]
fn diagnostic_event_clone() {
    let event = DiagnosticEvent::new(1, DiagnosticSeverity::Warning, "src", "warn")
        .with_command_id(10);
    let cloned = event.clone();
    assert_eq!(event.event_id, cloned.event_id);
    assert_eq!(event.command_id, cloned.command_id);
}

#[test]
fn diagnostic_event_debug() {
    let event = DiagnosticEvent::new(1, DiagnosticSeverity::Error, "src", "err");
    let debug = format!("{:?}", event);
    assert!(debug.contains("DiagnosticEvent"));
}

#[test]
fn diagnostic_event_serializes() {
    let event = DiagnosticEvent::new(1, DiagnosticSeverity::Warning, "my_source", "warning text");
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["event_id"], 1);
    assert!(json["source"].as_str().unwrap().contains("my_source"));
}

#[test]
fn diagnostic_event_serializes_with_command() {
    let event =
        DiagnosticEvent::new(5, DiagnosticSeverity::Error, "build", "compile error").with_command_id(99);
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["command_id"], 99);
}

#[test]
fn diagnostic_event_deserializes() {
    let json = r#"{
        "event_id": 10,
        "severity": "Info",
        "source": "test",
        "message": "hello",
        "timestamp": 12345,
        "command_id": null
    }"#;
    let event: DiagnosticEvent = serde_json::from_str(json).unwrap();
    assert_eq!(event.event_id, 10);
    assert_eq!(event.severity, DiagnosticSeverity::Info);
}

#[test]
fn diagnostic_event_timestamp_is_recent() {
    let event = DiagnosticEvent::new(1, DiagnosticSeverity::Info, "src", "msg");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    // Timestamp should be within last second
    assert!(now.saturating_sub(event.timestamp) < 1000);
}

// ============================================================================
// DiagnosticsPublisher tests
// ============================================================================

#[test]
fn publisher_new() {
    let pub_ = DiagnosticsPublisher::new();
    assert!(pub_.events().is_empty());
}

#[test]
fn publisher_default() {
    let pub_ = DiagnosticsPublisher::default();
    assert!(pub_.events().is_empty());
}

#[test]
fn publisher_publish_info() {
    let mut pub_ = DiagnosticsPublisher::new();
    let id = pub_.publish(DiagnosticSeverity::Info, "src1", "info msg");
    assert_eq!(id, 1);
    assert_eq!(pub_.events().len(), 1);
}

#[test]
fn publisher_publish_error() {
    let mut pub_ = DiagnosticsPublisher::new();
    let id = pub_.publish(DiagnosticSeverity::Error, "compiler", "fatal error");
    assert_eq!(id, 1);
}

#[test]
fn publisher_publish_multiple_events() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "msg1");
    pub_.publish(DiagnosticSeverity::Warning, "src", "msg2");
    pub_.publish(DiagnosticSeverity::Error, "src", "msg3");
    assert_eq!(pub_.events().len(), 3);
}

#[test]
fn publisher_event_ids_increment() {
    let mut pub_ = DiagnosticsPublisher::new();
    let id1 = pub_.publish(DiagnosticSeverity::Info, "src", "msg1");
    let id2 = pub_.publish(DiagnosticSeverity::Info, "src", "msg2");
    let id3 = pub_.publish(DiagnosticSeverity::Info, "src", "msg3");
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

#[test]
fn publisher_publish_with_command() {
    let mut pub_ = DiagnosticsPublisher::new();
    let id = pub_.publish_with_command(
        DiagnosticSeverity::Error,
        "build",
        "compile error",
        42,
    );
    assert_eq!(id, 1);
    assert_eq!(pub_.events().len(), 1);
    assert_eq!(pub_.events()[0].command_id, Some(42));
}

#[test]
fn publisher_events_for_command() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "msg1");
    pub_.publish_with_command(DiagnosticSeverity::Warning, "src", "cmd warning", 10);
    pub_.publish_with_command(DiagnosticSeverity::Error, "src", "cmd error", 10);
    pub_.publish_with_command(DiagnosticSeverity::Info, "src", "other cmd", 20);

    let cmd10_events = pub_.events_for_command(10);
    assert_eq!(cmd10_events.len(), 2);

    let cmd20_events = pub_.events_for_command(20);
    assert_eq!(cmd20_events.len(), 1);

    let cmd99_events = pub_.events_for_command(99);
    assert_eq!(cmd99_events.len(), 0);
}

#[test]
fn publisher_clear() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "msg1");
    pub_.publish(DiagnosticSeverity::Warning, "src", "msg2");
    assert_eq!(pub_.events().len(), 2);

    pub_.clear();
    assert!(pub_.events().is_empty());
}

#[test]
fn publisher_clear_and_publish_again() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "msg1");
    pub_.clear();
    let id = pub_.publish(DiagnosticSeverity::Error, "src", "msg2");
    assert_eq!(id, 2); // IDs continue incrementing
    assert_eq!(pub_.events().len(), 1);
}

#[test]
fn publisher_mixed_publish_methods() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "plain");
    pub_.publish_with_command(DiagnosticSeverity::Warning, "src", "with cmd", 5);
    pub_.publish(DiagnosticSeverity::Error, "src", "another plain");

    assert_eq!(pub_.events().len(), 3);
    assert_eq!(pub_.events()[0].command_id, None);
    assert_eq!(pub_.events()[1].command_id, Some(5));
    assert_eq!(pub_.events()[2].command_id, None);
}

#[test]
fn publisher_events_slice() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "msg");
    let events: &[DiagnosticEvent] = pub_.events();
    assert_eq!(events.len(), 1);
}

#[test]
fn publisher_severity_filter() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "src", "info");
    pub_.publish(DiagnosticSeverity::Warning, "src", "warn");
    pub_.publish(DiagnosticSeverity::Error, "src", "err");

    let error_count = pub_
        .events()
        .iter()
        .filter(|e| e.severity == DiagnosticSeverity::Error)
        .count();
    assert_eq!(error_count, 1);
}

#[test]
fn publisher_source_filter() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Info, "build", "build msg");
    pub_.publish(DiagnosticSeverity::Info, "lint", "lint msg");
    pub_.publish(DiagnosticSeverity::Info, "build", "another build msg");

    let build_count = pub_
        .events()
        .iter()
        .filter(|e| e.source == "build")
        .count();
    assert_eq!(build_count, 2);
}

#[test]
fn publisher_large_volume() {
    let mut pub_ = DiagnosticsPublisher::new();
    for i in 0..100 {
        pub_.publish(DiagnosticSeverity::Info, "src", &format!("msg {}", i));
    }
    assert_eq!(pub_.events().len(), 100);
    assert_eq!(pub_.events()[99].message, "msg 99");
}

#[test]
fn publisher_event_message_content() {
    let mut pub_ = DiagnosticsPublisher::new();
    pub_.publish(DiagnosticSeverity::Warning, "validator", "missing field 'name'");
    let event = &pub_.events()[0];
    assert!(event.message.contains("missing field"));
}
