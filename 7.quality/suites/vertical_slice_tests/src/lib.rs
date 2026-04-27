/// Vertical Slice End-to-End Tests
///
/// Tests for Properties 24-26:
/// - Property 24: Vertical Slice Completeness - each active slice has all 4 layers
/// - Property 25: Incomplete Slice Marking - incomplete slices are marked "future"
/// - Property 26: Vertical Slice End-to-End Testing - each active slice works end-to-end

pub const AUDIT_REPORT_PATH: &str = "../../data/generated/vertical_slice_audit.json";

/// Load the vertical slice audit report
pub fn load_audit_report() -> serde_json::Value {
    let content = std::fs::read_to_string(AUDIT_REPORT_PATH)
        .expect("Audit report should exist at expected path");
    serde_json::from_str(&content).expect("Audit report should be valid JSON")
}

/// Get all active slices from the audit report
pub fn get_active_slices() -> Vec<serde_json::Value> {
    let report = load_audit_report();
    report
        .get("slices")
        .and_then(|s| s.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|s| s.get("status").and_then(|v| v.as_str()) == Some("active"))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// Get all future slices from the audit report
pub fn get_future_slices() -> Vec<serde_json::Value> {
    let report = load_audit_report();
    report
        .get("slices")
        .and_then(|s| s.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|s| s.get("status").and_then(|v| v.as_str()) == Some("future"))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// Check if a slice has all four layers present
pub fn slice_has_all_layers(slice: &serde_json::Value) -> bool {
    slice.get("engine_present").and_then(|v| v.as_bool()) == Some(true)
        && slice.get("sdk_present").and_then(|v| v.as_bool()) == Some(true)
        && slice.get("tooling_present").and_then(|v| v.as_bool()) == Some(true)
        && slice.get("editor_present").and_then(|v| v.as_bool()) == Some(true)
}
