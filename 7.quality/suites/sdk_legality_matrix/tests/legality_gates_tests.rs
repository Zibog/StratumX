use legality_gates::*;
use sdk_compat::versions::CompatDomain;
use sdk_compat::{BridgeVersion, Capability, CompatibilityProfile};
use transport_policies::{default_transport_policy, FramingKind, MAX_PACKET_BYTES};

// ============================================================================
// LegalityGateId Tests
// ============================================================================

#[test]
fn test_legality_gate_id_new() {
    let id = LegalityGateId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_legality_gate_id_equality() {
    let a = LegalityGateId(1);
    let b = LegalityGateId(1);
    assert_eq!(a, b);
}

#[test]
fn test_legality_gate_id_ordering() {
    let a = LegalityGateId(1);
    let b = LegalityGateId(2);
    assert!(a < b);
}

#[test]
fn test_legality_gate_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = LegalityGateId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// LegalityGate Tests
// ============================================================================

#[test]
fn test_legality_gate_creation() {
    let gate = LegalityGate {
        legality_gate_id: LegalityGateId(1),
        gate_name: "ControlGate".to_string(),
        applies_to_domain: CompatDomain::Runtime,
        required_capability_set: vec![Capability::Controls],
        deny_reason: GateDenyReason::MissingCapability(Capability::Controls),
    };
    assert_eq!(gate.gate_name, "ControlGate");
    assert_eq!(gate.applies_to_domain, CompatDomain::Runtime);
    assert_eq!(gate.required_capability_set.len(), 1);
    assert!(matches!(
        gate.deny_reason,
        GateDenyReason::MissingCapability(Capability::Controls)
    ));
}

#[test]
fn test_legality_gate_empty_control_kinds() {
    let gate = LegalityGate {
        legality_gate_id: LegalityGateId(2),
        gate_name: "EmptyGate".to_string(),
        applies_to_domain: CompatDomain::Diagnostics,
        required_capability_set: vec![],
        deny_reason: GateDenyReason::PolicyViolation,
    };
    assert_eq!(gate.applies_to_domain, CompatDomain::Diagnostics);
    assert!(gate.required_capability_set.is_empty());
}

#[test]
fn test_legality_gate_equality() {
    let gate_a = LegalityGate {
        legality_gate_id: LegalityGateId(1),
        gate_name: "TestGate".to_string(),
        applies_to_domain: CompatDomain::Runtime,
        required_capability_set: vec![Capability::Controls],
        deny_reason: GateDenyReason::PolicyViolation,
    };
    let gate_b = LegalityGate {
        legality_gate_id: LegalityGateId(1),
        gate_name: "TestGate".to_string(),
        applies_to_domain: CompatDomain::Runtime,
        required_capability_set: vec![Capability::Controls],
        deny_reason: GateDenyReason::PolicyViolation,
    };
    assert_eq!(gate_a, gate_b);
}

#[test]
fn test_legality_gate_inequality() {
    let gate_a = LegalityGate {
        legality_gate_id: LegalityGateId(1),
        gate_name: "GateA".to_string(),
        applies_to_domain: CompatDomain::Runtime,
        required_capability_set: vec![],
        deny_reason: GateDenyReason::PolicyViolation,
    };
    let gate_b = LegalityGate {
        legality_gate_id: LegalityGateId(1),
        gate_name: "GateB".to_string(),
        applies_to_domain: CompatDomain::Runtime,
        required_capability_set: vec![],
        deny_reason: GateDenyReason::PolicyViolation,
    };
    assert_ne!(gate_a, gate_b);
}

// ============================================================================
// compatibility_verdict Function Tests
// ============================================================================

#[test]
fn test_compatibility_verdict_allow_supported() {
    let version = BridgeVersion::new(1, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Snapshots, Capability::Controls];
    let profile = CompatibilityProfile::ToolRuntime;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Allow
    ));
}

#[test]
fn test_compatibility_verdict_deny_version_too_old() {
    let version = BridgeVersion::new(0, 9, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Snapshots, Capability::Controls];
    let profile = CompatibilityProfile::ToolRuntime;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Deny
    ));
    assert!(verdict
        .reason_code_set
        .iter()
        .any(|r| matches!(r, sdk_compat::CompatibilityReasonCode::VersionTooOld)));
}

#[test]
fn test_compatibility_verdict_deny_missing_capability() {
    let version = BridgeVersion::new(1, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![]; // No capabilities
    let profile = CompatibilityProfile::ToolRuntime;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Deny
    ));
    assert!(verdict
        .reason_code_set
        .iter()
        .any(|r| matches!(r, sdk_compat::CompatibilityReasonCode::MissingCapability(_))));
}

#[test]
fn test_compatibility_verdict_editor_surface_profile() {
    let version = BridgeVersion::new(1, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Snapshots, Capability::Observations];
    let profile = CompatibilityProfile::EditorSurface;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Allow
    ));
}

#[test]
fn test_compatibility_verdict_automation_profile() {
    let version = BridgeVersion::new(1, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Snapshots, Capability::Metrics];
    let profile = CompatibilityProfile::Automation;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Allow
    ));
}

#[test]
fn test_compatibility_verdict_diagnostics_profile() {
    let version = BridgeVersion::new(1, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Observations, Capability::Metrics];
    let profile = CompatibilityProfile::Diagnostics;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Allow
    ));
}

#[test]
fn test_compatibility_verdict_partial_capabilities() {
    let version = BridgeVersion::new(1, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Snapshots]; // Missing Controls
    let profile = CompatibilityProfile::ToolRuntime;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Deny
    ));
}

#[test]
fn test_compatibility_verdict_higher_version() {
    let version = BridgeVersion::new(2, 0, 0);
    let baseline = BridgeVersion::new(1, 0, 0);
    let capabilities = vec![Capability::Snapshots, Capability::Controls];
    let profile = CompatibilityProfile::ToolRuntime;

    let verdict = compatibility_verdict(version, &capabilities, profile, baseline);
    // Higher version should still be allowed if capabilities match
    assert!(matches!(
        verdict.verdict_state,
        sdk_compat::CompatibilityVerdictState::Allow
    ));
}

// ============================================================================
// transport_legality Function Tests
// ============================================================================

#[test]
fn test_transport_legality_legal_ordered_control() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    let verdict = transport_legality(&policy, 100, false, false);
    assert!(matches!(verdict, Ok(sdk_compat::LegalityVerdict::Legal)));
}

#[test]
fn test_transport_legality_illegal_payload_too_large() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    let verdict = transport_legality(&policy, MAX_PACKET_BYTES + 1, false, false);
    assert!(verdict.is_err());
}

#[test]
fn test_transport_legality_illegal_metric_not_allowed() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    let verdict = transport_legality(&policy, 100, true, false);
    assert!(verdict.is_err());
}

#[test]
fn test_transport_legality_illegal_artifact_not_allowed() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    let verdict = transport_legality(&policy, 100, false, true);
    assert!(verdict.is_err());
}

#[test]
fn test_transport_legality_metrics_only_legal() {
    let policy = default_transport_policy(FramingKind::MetricsOnly);
    let verdict = transport_legality(&policy, 0, true, false);
    assert!(matches!(verdict, Ok(sdk_compat::LegalityVerdict::Legal)));
}

#[test]
fn test_transport_legality_metrics_only_illegal_with_payload() {
    let policy = default_transport_policy(FramingKind::MetricsOnly);
    let verdict = transport_legality(&policy, 100, false, false);
    assert!(verdict.is_err());
}

#[test]
fn test_transport_legality_artifact_only_legal() {
    let policy = default_transport_policy(FramingKind::ArtifactOnly);
    let verdict = transport_legality(&policy, 0, false, true);
    assert!(matches!(verdict, Ok(sdk_compat::LegalityVerdict::Legal)));
}

#[test]
fn test_transport_legality_artifact_only_illegal_no_artifact() {
    let policy = default_transport_policy(FramingKind::ArtifactOnly);
    let verdict = transport_legality(&policy, 0, false, false);
    assert!(verdict.is_err());
}

#[test]
fn test_transport_legality_artifact_only_illegal_with_payload() {
    let policy = default_transport_policy(FramingKind::ArtifactOnly);
    // ArtifactOnly allows 128 bytes max
    let verdict = transport_legality(&policy, 200, false, true);
    assert!(verdict.is_err());
}

#[test]
fn test_transport_legality_zero_payload_always_legal() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    let verdict = transport_legality(&policy, 0, false, false);
    assert!(matches!(verdict, Ok(sdk_compat::LegalityVerdict::Legal)));
}

#[test]
fn test_transport_legality_exact_max_payload() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    let verdict = transport_legality(&policy, MAX_PACKET_BYTES, false, false);
    assert!(matches!(verdict, Ok(sdk_compat::LegalityVerdict::Legal)));
}

#[test]
fn test_transport_legality_bounded_preview_with_artifact() {
    let policy = default_transport_policy(FramingKind::BoundedPreview);
    let verdict = transport_legality(&policy, 1000, false, true);
    assert!(matches!(verdict, Ok(sdk_compat::LegalityVerdict::Legal)));
}

// ============================================================================
// Legality Verdict Type Tests
// ============================================================================

#[test]
fn test_legality_verdict_variants() {
    assert_ne!(
        sdk_compat::LegalityVerdict::Legal as u8,
        sdk_compat::LegalityVerdict::Illegal as u8
    );
}

#[test]
fn test_legality_verdict_equality() {
    let a = sdk_compat::LegalityVerdict::Legal;
    let b = sdk_compat::LegalityVerdict::Legal;
    assert_eq!(a, b);
}
