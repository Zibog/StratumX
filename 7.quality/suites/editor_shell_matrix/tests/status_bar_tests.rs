use stratumx_editor_l8_0_editor_shell::status_bar::{
    GeneratedQualitySummary, MessageType, StatusBar,
};

#[test]
fn test_status_bar_message() {
    let mut status = StatusBar::new();

    status.set_message("Test message", MessageType::Info);
    assert_eq!(status.message, Some("Test message".to_string()));
    assert_eq!(status.message_type, MessageType::Info);

    status.set_message("Error!", MessageType::Error);
    assert_eq!(status.message_type, MessageType::Error);

    status.clear_message();
    assert!(status.message.is_none());
}

#[test]
fn test_performance_update() {
    let mut status = StatusBar::new();

    status.update_performance(60.0, 16.67);
    assert_eq!(status.fps, 60.0);
    assert_eq!(status.frame_time_ms, 16.67);
}

#[test]
fn test_set_quality_summary() {
    let mut status = StatusBar::new();
    status.set_quality_summary(Some(GeneratedQualitySummary {
        verify_status: Some("passed".to_string()),
        smoke_status: Some("passed".to_string()),
        full_status: Some("passed".to_string()),
        declared_tests: 10,
        route_coverage: 20,
        workspace_packages: 30,
    }));

    assert_eq!(
        status
            .quality_summary
            .as_ref()
            .and_then(|summary| summary.full_status.clone()),
        Some("passed".to_string())
    );
}
