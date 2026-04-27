// Diagnostics Operations

use super::diagnostics_types::{Diagnostic, Severity};
use editor_dto_law::Posture;
use std::time::SystemTime;

/// Reports host truth posture as a diagnostic.
/// This is an impl block template for EditorHost or similar host type.
pub fn report_host_truth(
    collect_fn: &mut impl FnMut(Diagnostic),
    posture: Posture,
    message: Option<String>,
) {
    let msg = message.unwrap_or_else(|| format!("Host posture: {:?}", posture));
    let severity = match posture {
        Posture::Healthy => Severity::Info,
        Posture::Degraded => Severity::Warning,
        Posture::Failed => Severity::Error,
        Posture::Unknown => Severity::Warning,
    };
    collect_fn(Diagnostic {
        severity,
        domain: "host".to_string(),
        message: msg,
        timestamp: SystemTime::now(),
    });
}

/// Gets a diagnostics summary string.
pub fn get_diagnostics_summary(count: usize) -> String {
    format!("Diagnostics count: {}", count)
}
