use sdk_compat::*;
use std::hash::Hasher;

// ============================================================================
// BridgeVersion Tests
// ============================================================================

#[test]
fn test_bridge_version_new() {
    let v = BridgeVersion::new(1, 2, 3);
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 3);
}

#[test]
fn test_bridge_version_equality() {
    let a = BridgeVersion::new(1, 0, 0);
    let b = BridgeVersion::new(1, 0, 0);
    let c = BridgeVersion::new(1, 0, 1);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_bridge_version_ordering() {
    let v1 = BridgeVersion::new(1, 0, 0);
    let v2 = BridgeVersion::new(1, 1, 0);
    let v3 = BridgeVersion::new(2, 0, 0);
    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);
}

#[test]
fn test_bridge_version_const_new() {
    const V: BridgeVersion = BridgeVersion::new(0, 0, 1);
    assert_eq!(V.major, 0);
    assert_eq!(V.minor, 0);
    assert_eq!(V.patch, 1);
}

#[test]
fn test_bridge_version_zero() {
    let v = BridgeVersion::new(0, 0, 0);
    assert_eq!(v.major, 0);
}

#[test]
fn test_bridge_version_max_values() {
    let v = BridgeVersion::new(u16::MAX, u16::MAX, u16::MAX);
    assert_eq!(v.major, u16::MAX);
}

// ============================================================================
// CompatVersionId Tests
// ============================================================================

#[test]
fn test_compat_version_id_new() {
    let id = CompatVersionId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_compat_version_id_equality() {
    let a = CompatVersionId(1);
    let b = CompatVersionId(1);
    assert_eq!(a, b);
}

#[test]
fn test_compat_version_id_ordering() {
    let a = CompatVersionId(1);
    let b = CompatVersionId(2);
    assert!(a < b);
}

#[test]
fn test_compat_version_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = CompatVersionId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// Capability Tests
// ============================================================================

#[test]
fn test_capability_variants_all_distinct() {
    let caps = [
        Capability::Snapshots,
        Capability::Observations,
        Capability::Metrics,
        Capability::ArtifactRefs,
        Capability::Controls,
    ];
    for i in 0..caps.len() {
        for j in (i + 1)..caps.len() {
            assert_ne!(
                caps[i], caps[j],
                "Capabilities at {} and {} should differ",
                i, j
            );
        }
    }
}

#[test]
fn test_capability_equality() {
    let a = Capability::Snapshots;
    let b = Capability::Snapshots;
    assert_eq!(a, b);
}

#[test]
fn test_capability_ordering() {
    assert!(Capability::Snapshots < Capability::Observations);
    assert!(Capability::Metrics < Capability::ArtifactRefs);
}

#[test]
fn test_capability_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let cap = Capability::Controls;
    let mut h1 = DefaultHasher::new();
    cap.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    cap.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn test_compat_capability_id_new() {
    let id = CompatCapabilityId(10);
    assert_eq!(id.0, 10);
}

#[test]
fn test_capability_fact_creation() {
    let fact = CapabilityFact {
        capability_id: CompatCapabilityId(1),
        capability_name: "Snapshots".to_string(),
        default_state: CapabilityDefaultState::Allowed,
        deprecation_note: None,
    };
    assert_eq!(fact.capability_name, "Snapshots");
    assert!(fact.deprecation_note.is_none());
}

#[test]
fn test_capability_fact_with_deprecation() {
    let fact = CapabilityFact {
        capability_id: CompatCapabilityId(2),
        capability_name: "LegacyFeature".to_string(),
        default_state: CapabilityDefaultState::Denied,
        deprecation_note: Some("Use new feature instead".to_string()),
    };
    assert!(fact.deprecation_note.is_some());
}

#[test]
fn test_capability_default_state_variants() {
    assert_ne!(
        CapabilityDefaultState::Allowed as u8,
        CapabilityDefaultState::Denied as u8
    );
}

// ============================================================================
// CompatibilityProfile Tests
// ============================================================================

#[test]
fn test_compatibility_profile_variants_all_distinct() {
    let profiles = [
        CompatibilityProfile::ToolRuntime,
        CompatibilityProfile::EditorSurface,
        CompatibilityProfile::Automation,
        CompatibilityProfile::Diagnostics,
    ];
    for i in 0..profiles.len() {
        for j in (i + 1)..profiles.len() {
            assert_ne!(profiles[i], profiles[j]);
        }
    }
}

#[test]
fn test_compatibility_profile_equality() {
    let a = CompatibilityProfile::ToolRuntime;
    let b = CompatibilityProfile::ToolRuntime;
    assert_eq!(a, b);
}

#[test]
fn test_compatibility_profile_ordering() {
    assert!(CompatibilityProfile::ToolRuntime < CompatibilityProfile::EditorSurface);
}

#[test]
fn test_compat_profile_id_new() {
    let id = CompatProfileId(5);
    assert_eq!(id.0, 5);
}

#[test]
fn test_profile_fact_creation() {
    let fact = ProfileFact {
        profile_id: CompatProfileId(1),
        profile_name: "ToolRuntime".to_string(),
        allowed_version_set: vec![CompatVersionId(1), CompatVersionId(2)],
        capability_set: vec![Capability::Snapshots, Capability::Controls],
        capability_id_set: vec![CompatCapabilityId(1)],
        fallback_profile_id: None,
    };
    assert_eq!(fact.profile_name, "ToolRuntime");
    assert_eq!(fact.allowed_version_set.len(), 2);
    assert_eq!(fact.capability_set.len(), 2);
}

#[test]
fn test_profile_fact_with_fallback() {
    let fact = ProfileFact {
        profile_id: CompatProfileId(2),
        profile_name: "Diagnostics".to_string(),
        allowed_version_set: vec![],
        capability_set: vec![Capability::Observations],
        capability_id_set: vec![],
        fallback_profile_id: Some(CompatProfileId(1)),
    };
    assert!(fact.fallback_profile_id.is_some());
}

// ============================================================================
// CompatibilityVerdict Tests
// ============================================================================

#[test]
fn test_verdict_state_variants() {
    assert_ne!(
        CompatibilityVerdictState::Allow as u8,
        CompatibilityVerdictState::Deny as u8
    );
    assert_ne!(
        CompatibilityVerdictState::Warn as u8,
        CompatibilityVerdictState::Allow as u8
    );
}

#[test]
fn test_compatibility_verdict_creation() {
    let verdict = CompatibilityVerdict {
        compat_verdict_id: CompatVerdictId(1),
        evaluated_profile_id: CompatProfileId(1),
        evaluated_version_id: CompatVersionId(1),
        verdict_state: CompatibilityVerdictState::Allow,
        reason_code_set: vec![CompatibilityReasonCode::Supported],
    };
    assert!(matches!(
        verdict.verdict_state,
        CompatibilityVerdictState::Allow
    ));
    assert_eq!(verdict.reason_code_set.len(), 1);
}

#[test]
fn test_compatibility_verdict_deny() {
    let verdict = CompatibilityVerdict {
        compat_verdict_id: CompatVerdictId(2),
        evaluated_profile_id: CompatProfileId(2),
        evaluated_version_id: CompatVersionId(1),
        verdict_state: CompatibilityVerdictState::Deny,
        reason_code_set: vec![CompatibilityReasonCode::VersionTooOld],
    };
    assert!(matches!(
        verdict.verdict_state,
        CompatibilityVerdictState::Deny
    ));
}

#[test]
fn test_compatibility_verdict_equality() {
    let a = CompatibilityVerdict {
        compat_verdict_id: CompatVerdictId(0),
        evaluated_profile_id: CompatProfileId(1),
        evaluated_version_id: CompatVersionId(1),
        verdict_state: CompatibilityVerdictState::Allow,
        reason_code_set: vec![CompatibilityReasonCode::Supported],
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_reason_code_version_too_old() {
    let code = CompatibilityReasonCode::VersionTooOld;
    let verdict = CompatibilityVerdict {
        compat_verdict_id: CompatVerdictId(0),
        evaluated_profile_id: CompatProfileId(1),
        evaluated_version_id: CompatVersionId(0),
        verdict_state: CompatibilityVerdictState::Deny,
        reason_code_set: vec![code],
    };
    assert_eq!(verdict.reason_code_set.len(), 1);
}

#[test]
fn test_reason_code_missing_capability() {
    let code = CompatibilityReasonCode::MissingCapability(Capability::Snapshots);
    assert!(matches!(
        code,
        CompatibilityReasonCode::MissingCapability(_)
    ));
}

#[test]
fn test_reason_code_supported() {
    let code = CompatibilityReasonCode::Supported;
    assert!(matches!(code, CompatibilityReasonCode::Supported));
}

#[test]
fn test_compat_verdict_id_equality() {
    let a = CompatVerdictId(1);
    let b = CompatVerdictId(1);
    assert_eq!(a, b);
}

// ============================================================================
// SupportState and VersionFact Tests
// ============================================================================

#[test]
fn test_support_state_variants() {
    assert_ne!(
        SupportState::Supported as u8,
        SupportState::Deprecated as u8
    );
    assert_ne!(SupportState::Denied as u8, SupportState::Supported as u8);
}

#[test]
fn test_version_fact_creation() {
    let fact = VersionFact {
        compat_version_id: CompatVersionId(1),
        wire_version: BridgeVersion::new(1, 0, 0),
        schema_generation: 1,
        supersedes_version_id: None,
        support_state: SupportState::Supported,
    };
    assert_eq!(fact.schema_generation, 1);
    assert!(fact.supersedes_version_id.is_none());
}

#[test]
fn test_version_fact_with_supersedes() {
    let fact = VersionFact {
        compat_version_id: CompatVersionId(2),
        wire_version: BridgeVersion::new(2, 0, 0),
        schema_generation: 2,
        supersedes_version_id: Some(CompatVersionId(1)),
        support_state: SupportState::Supported,
    };
    assert!(fact.supersedes_version_id.is_some());
}

#[test]
fn test_version_fact_deprecated() {
    let fact = VersionFact {
        compat_version_id: CompatVersionId(3),
        wire_version: BridgeVersion::new(0, 9, 0),
        schema_generation: 1,
        supersedes_version_id: None,
        support_state: SupportState::Deprecated,
    };
    assert!(matches!(fact.support_state, SupportState::Deprecated));
}

#[test]
fn test_version_fact_denied() {
    let fact = VersionFact {
        compat_version_id: CompatVersionId(4),
        wire_version: BridgeVersion::new(0, 1, 0),
        schema_generation: 1,
        supersedes_version_id: None,
        support_state: SupportState::Denied,
    };
    assert!(matches!(fact.support_state, SupportState::Denied));
}

// ============================================================================
// Canonical Level Constant
// ============================================================================

#[test]
fn test_canonical_level_constant() {
    assert_eq!(CANONICAL_LEVEL, "l5-compat");
}

// ============================================================================
// BridgeVersion Hash Tests
// ============================================================================

#[test]
fn test_bridge_version_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let v = BridgeVersion::new(1, 2, 3);
    let mut h1 = DefaultHasher::new();
    v.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    v.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn test_bridge_version_different_versions_different_hashes() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    let v1 = BridgeVersion::new(1, 0, 0);
    let v2 = BridgeVersion::new(2, 0, 0);
    let mut h1 = DefaultHasher::new();
    v1.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    v2.hash(&mut h2);
    // May collide but unlikely; not asserted
    let _ = (h1.finish(), h2.finish());
}

// ============================================================================
// Profile/Compatibility Edge Cases
// ============================================================================

#[test]
fn test_profile_fact_empty_sets() {
    let fact = ProfileFact {
        profile_id: CompatProfileId(99),
        profile_name: "Empty".to_string(),
        allowed_version_set: vec![],
        capability_set: vec![],
        capability_id_set: vec![],
        fallback_profile_id: None,
    };
    assert!(fact.allowed_version_set.is_empty());
    assert!(fact.capability_set.is_empty());
}

#[test]
fn test_compatibility_verdict_multiple_reasons() {
    let verdict = CompatibilityVerdict {
        compat_verdict_id: CompatVerdictId(1),
        evaluated_profile_id: CompatProfileId(1),
        evaluated_version_id: CompatVersionId(0),
        verdict_state: CompatibilityVerdictState::Deny,
        reason_code_set: vec![
            CompatibilityReasonCode::VersionTooOld,
            CompatibilityReasonCode::MissingCapability(Capability::Snapshots),
            CompatibilityReasonCode::MissingCapability(Capability::Controls),
        ],
    };
    assert_eq!(verdict.reason_code_set.len(), 3);
}
