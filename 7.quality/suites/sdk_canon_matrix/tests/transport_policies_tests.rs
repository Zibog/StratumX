use sdk_compat::{BridgeVersion, CompatibilityProfile};
use transport_policies::*;

// ============================================================================
// Constants
// ============================================================================

#[test]
fn test_canonical_level_constant() {
    assert_eq!(CANONICAL_LEVEL, "L5.8");
}

#[test]
fn test_max_packet_bytes_constant() {
    assert_eq!(MAX_PACKET_BYTES, 16 * 1024);
    assert_eq!(MAX_PACKET_BYTES, 16384);
}

// ============================================================================
// TransportPolicyId Tests
// ============================================================================

#[test]
fn test_transport_policy_id_new() {
    let id = TransportPolicyId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_transport_policy_id_equality() {
    let a = TransportPolicyId(1);
    let b = TransportPolicyId(1);
    let c = TransportPolicyId(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_transport_policy_id_ordering() {
    let a = TransportPolicyId(1);
    let b = TransportPolicyId(2);
    assert!(a < b);
}

#[test]
fn test_transport_policy_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = TransportPolicyId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// FramingKind Tests
// ============================================================================

#[test]
fn test_framing_kind_variants_all_distinct() {
    let kinds = [
        FramingKind::OrderedControl,
        FramingKind::BoundedPreview,
        FramingKind::MetricsOnly,
        FramingKind::ArtifactOnly,
    ];
    for i in 0..kinds.len() {
        for j in (i + 1)..kinds.len() {
            assert_ne!(kinds[i] as u8, kinds[j] as u8);
        }
    }
}

#[test]
fn test_framing_kind_equality() {
    let a = FramingKind::OrderedControl;
    let b = FramingKind::OrderedControl;
    assert_eq!(a, b);
}

// ============================================================================
// RetryClass Tests
// ============================================================================

#[test]
fn test_retry_class_variants() {
    assert_ne!(RetryClass::None as u8, RetryClass::Bounded as u8);
}

#[test]
fn test_retry_class_equality() {
    let a = RetryClass::Bounded;
    let b = RetryClass::Bounded;
    assert_eq!(a, b);
}

// ============================================================================
// TransportPolicy Creation Tests
// ============================================================================

#[test]
fn test_transport_policy_ordered_control() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    assert_eq!(policy.transport_policy_id, TransportPolicyId(1));
    assert!(matches!(policy.framing_kind, FramingKind::OrderedControl));
    assert_eq!(policy.max_payload_bytes, MAX_PACKET_BYTES);
    assert!(matches!(policy.retry_class, RetryClass::Bounded));
    assert_eq!(policy.batching_window_ticks, 1);
    assert!(!policy.allow_metrics);
    assert!(!policy.allow_artifact_refs);
}

#[test]
fn test_transport_policy_bounded_preview() {
    let policy = default_transport_policy(FramingKind::BoundedPreview);
    assert_eq!(policy.transport_policy_id, TransportPolicyId(2));
    assert!(matches!(policy.framing_kind, FramingKind::BoundedPreview));
    assert_eq!(policy.max_payload_bytes, MAX_PACKET_BYTES / 2);
    assert!(matches!(policy.retry_class, RetryClass::Bounded));
    assert_eq!(policy.batching_window_ticks, 2);
    assert!(!policy.allow_metrics);
    assert!(policy.allow_artifact_refs);
}

#[test]
fn test_transport_policy_metrics_only() {
    let policy = default_transport_policy(FramingKind::MetricsOnly);
    assert_eq!(policy.transport_policy_id, TransportPolicyId(3));
    assert!(matches!(policy.framing_kind, FramingKind::MetricsOnly));
    assert_eq!(policy.max_payload_bytes, 0);
    assert!(matches!(policy.retry_class, RetryClass::None));
    assert!(policy.allow_metrics);
    assert!(!policy.allow_artifact_refs);
}

#[test]
fn test_transport_policy_artifact_only() {
    let policy = default_transport_policy(FramingKind::ArtifactOnly);
    assert_eq!(policy.transport_policy_id, TransportPolicyId(4));
    assert!(matches!(policy.framing_kind, FramingKind::ArtifactOnly));
    assert_eq!(policy.max_payload_bytes, 128);
    assert!(matches!(policy.retry_class, RetryClass::None));
    assert!(!policy.allow_metrics);
    assert!(policy.allow_artifact_refs);
}

#[test]
fn test_transport_policy_default_compat_none() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    assert!(policy.compat_profile.is_none());
    assert!(policy.compat_version.is_none());
}

#[test]
fn test_transport_policy_equality() {
    let a = default_transport_policy(FramingKind::OrderedControl);
    let b = default_transport_policy(FramingKind::OrderedControl);
    assert_eq!(a, b);
}

#[test]
fn test_transport_policy_inequality_different_framing() {
    let a = default_transport_policy(FramingKind::OrderedControl);
    let b = default_transport_policy(FramingKind::MetricsOnly);
    assert_ne!(a, b);
}

// ============================================================================
// TransportPolicy Manual Construction
// ============================================================================

#[test]
fn test_transport_policy_manual_with_profile() {
    let policy = TransportPolicy {
        transport_policy_id: TransportPolicyId(100),
        framing_kind: FramingKind::OrderedControl,
        max_payload_bytes: 4096,
        retry_class: RetryClass::None,
        batching_window_ticks: 5,
        delivery_scope: DeliveryScope::PointToPoint,
        priority: PriorityLevel::High,
        rate_tier: RateTier::Throttled {
            max_packets_per_second: 120,
        },
        compat_profile: Some(CompatibilityProfile::ToolRuntime),
        compat_version: Some(BridgeVersion::new(1, 0, 0)),
        allow_metrics: true,
        allow_artifact_refs: true,
    };
    assert_eq!(policy.transport_policy_id, TransportPolicyId(100));
    assert_eq!(policy.max_payload_bytes, 4096);
    assert!(policy.compat_profile.is_some());
    assert!(policy.compat_version.is_some());
    assert!(policy.allow_metrics);
    assert!(policy.allow_artifact_refs);
}

#[test]
fn test_transport_policy_all_fields_access() {
    let policy = default_transport_policy(FramingKind::BoundedPreview);
    // Verify all fields are accessible
    let _ = policy.transport_policy_id;
    let _ = policy.framing_kind;
    let _ = policy.max_payload_bytes;
    let _ = policy.retry_class;
    let _ = policy.batching_window_ticks;
    let _ = policy.compat_profile;
    let _ = policy.compat_version;
    let _ = policy.allow_metrics;
    let _ = policy.allow_artifact_refs;
}

// ============================================================================
// Payload Size Edge Cases
// ============================================================================

#[test]
fn test_transport_policy_zero_payload() {
    let policy = default_transport_policy(FramingKind::MetricsOnly);
    assert_eq!(policy.max_payload_bytes, 0);
}

#[test]
fn test_transport_policy_max_payload() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    assert_eq!(policy.max_payload_bytes, 16384);
}

#[test]
fn test_transport_policy_half_max_payload() {
    let policy = default_transport_policy(FramingKind::BoundedPreview);
    assert_eq!(policy.max_payload_bytes, 8192);
}

// ============================================================================
// Framing Kind Specific Behavior
// ============================================================================

#[test]
fn test_ordered_control_has_bounded_retry() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    assert!(matches!(policy.retry_class, RetryClass::Bounded));
}

#[test]
fn test_metrics_only_no_retry() {
    let policy = default_transport_policy(FramingKind::MetricsOnly);
    assert!(matches!(policy.retry_class, RetryClass::None));
}

#[test]
fn test_artifact_only_no_retry() {
    let policy = default_transport_policy(FramingKind::ArtifactOnly);
    assert!(matches!(policy.retry_class, RetryClass::None));
}

// ============================================================================
// Policy ID Uniqueness
// ============================================================================

#[test]
fn test_all_default_policies_have_unique_ids() {
    let policies = [
        default_transport_policy(FramingKind::OrderedControl),
        default_transport_policy(FramingKind::BoundedPreview),
        default_transport_policy(FramingKind::MetricsOnly),
        default_transport_policy(FramingKind::ArtifactOnly),
    ];
    for i in 0..policies.len() {
        for j in (i + 1)..policies.len() {
            assert_ne!(
                policies[i].transport_policy_id, policies[j].transport_policy_id,
                "Policies {} and {} have the same ID",
                i, j
            );
        }
    }
}

// ============================================================================
// Batching Window Tests
// ============================================================================

#[test]
fn test_ordered_control_batching_window() {
    let policy = default_transport_policy(FramingKind::OrderedControl);
    assert_eq!(policy.batching_window_ticks, 1);
}

#[test]
fn test_bounded_preview_batching_window() {
    let policy = default_transport_policy(FramingKind::BoundedPreview);
    assert_eq!(policy.batching_window_ticks, 2);
}
