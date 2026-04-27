use engine_handle_refs::{ObjectHandle, RuntimeHandle, SessionHandle};
use legality_gates::LegalityGateId;
use link_ingress_controls::*;

// ============================================================================
// IngressControlEnvelopeId Tests
// ============================================================================

#[test]
fn test_envelope_id_new() {
    let id = IngressControlEnvelopeId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_envelope_id_equality() {
    let a = IngressControlEnvelopeId(1);
    let b = IngressControlEnvelopeId(1);
    assert_eq!(a, b);
}

#[test]
fn test_envelope_id_ordering() {
    let a = IngressControlEnvelopeId(1);
    let b = IngressControlEnvelopeId(2);
    assert!(a < b);
}

#[test]
fn test_envelope_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = IngressControlEnvelopeId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// IngressControlKind Tests
// ============================================================================

#[test]
fn test_control_kind_set_label() {
    let kind = IngressControlKind::SetLabel {
        label: "my-label".to_string(),
    };
    assert!(matches!(kind, IngressControlKind::SetLabel { .. }));
}

#[test]
fn test_control_kind_set_field() {
    let kind = IngressControlKind::SetField {
        key: "key1".to_string(),
        value: "value1".to_string(),
    };
    assert!(matches!(kind, IngressControlKind::SetField { .. }));
}

#[test]
fn test_control_kind_clear_field() {
    let kind = IngressControlKind::ClearField {
        key: "key1".to_string(),
    };
    assert!(matches!(kind, IngressControlKind::ClearField { .. }));
}

#[test]
fn test_control_kind_add_tag() {
    let kind = IngressControlKind::AddTag {
        tag: "tag1".to_string(),
    };
    assert!(matches!(kind, IngressControlKind::AddTag { .. }));
}

#[test]
fn test_control_kind_remove_tag() {
    let kind = IngressControlKind::RemoveTag {
        tag: "tag1".to_string(),
    };
    assert!(matches!(kind, IngressControlKind::RemoveTag { .. }));
}

#[test]
fn test_control_kind_retire_object() {
    let kind = IngressControlKind::RetireObject;
    assert!(matches!(kind, IngressControlKind::RetireObject));
}

#[test]
fn test_control_kind_restore_object() {
    let kind = IngressControlKind::RestoreObject;
    assert!(matches!(kind, IngressControlKind::RestoreObject));
}

#[test]
fn test_control_kind_refresh_snapshot() {
    let kind = IngressControlKind::RefreshSnapshot;
    assert!(matches!(kind, IngressControlKind::RefreshSnapshot));
}

#[test]
fn test_control_kind_equality() {
    let a = IngressControlKind::SetLabel {
        label: "same".to_string(),
    };
    let b = IngressControlKind::SetLabel {
        label: "same".to_string(),
    };
    assert_eq!(a, b);
}

#[test]
fn test_control_kind_inequality() {
    let a = IngressControlKind::SetLabel {
        label: "label-a".to_string(),
    };
    let b = IngressControlKind::SetLabel {
        label: "label-b".to_string(),
    };
    assert_ne!(a, b);
}

#[test]
fn test_control_kind_different_variants_not_equal() {
    let a = IngressControlKind::SetLabel {
        label: "x".to_string(),
    };
    let b = IngressControlKind::RetireObject;
    assert_ne!(a, b);
}

#[test]
fn test_control_kind_all_variants_distinct() {
    let kinds = [
        IngressControlKind::SetLabel {
            label: "test".to_string(),
        },
        IngressControlKind::SetField {
            key: "k".to_string(),
            value: "v".to_string(),
        },
        IngressControlKind::ClearField {
            key: "k".to_string(),
        },
        IngressControlKind::AddTag {
            tag: "t".to_string(),
        },
        IngressControlKind::RemoveTag {
            tag: "t".to_string(),
        },
        IngressControlKind::RetireObject,
        IngressControlKind::RestoreObject,
        IngressControlKind::RefreshSnapshot,
    ];
    for i in 0..kinds.len() {
        for j in (i + 1)..kinds.len() {
            assert_ne!(kinds[i], kinds[j], "Kind {} should differ from kind {}", i, j);
        }
    }
}

// ============================================================================
// BridgeControl Tests
// ============================================================================

#[test]
fn test_bridge_control_creation() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(100),
        control_kind: IngressControlKind::SetLabel {
            label: "test".to_string(),
        },
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 0,
        legality_gate_id: LegalityGateId(1),
    };
    assert_eq!(control.ingress_control_envelope_id, IngressControlEnvelopeId(100));
    assert!(control.target_object_handle.is_none());
    assert_eq!(control.submission_order_key, 0);
}

#[test]
fn test_bridge_control_with_object_target() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let obj = ObjectHandle::new(42);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(1),
        control_kind: IngressControlKind::RetireObject,
        target_object_handle: Some(obj),
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 10,
        legality_gate_id: LegalityGateId(1),
    };
    assert!(control.target_object_handle.is_some());
    assert_eq!(control.submission_order_key, 10);
}

#[test]
fn test_bridge_control_set_field() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(1),
        control_kind: IngressControlKind::SetField {
            key: "health".to_string(),
            value: "100".to_string(),
        },
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 1,
        legality_gate_id: LegalityGateId(0),
    };
    if let IngressControlKind::SetField { key, value } = &control.control_kind {
        assert_eq!(key, "health");
        assert_eq!(value, "100");
    } else {
        panic!("Expected SetField variant");
    }
}

#[test]
fn test_bridge_control_clear_field() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(2),
        control_kind: IngressControlKind::ClearField {
            key: "temp_field".to_string(),
        },
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 2,
        legality_gate_id: LegalityGateId(0),
    };
    if let IngressControlKind::ClearField { key } = &control.control_kind {
        assert_eq!(key, "temp_field");
    } else {
        panic!("Expected ClearField variant");
    }
}

#[test]
fn test_bridge_control_add_tag() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(3),
        control_kind: IngressControlKind::AddTag {
            tag: "urgent".to_string(),
        },
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 3,
        legality_gate_id: LegalityGateId(1),
    };
    if let IngressControlKind::AddTag { tag } = &control.control_kind {
        assert_eq!(tag, "urgent");
    } else {
        panic!("Expected AddTag variant");
    }
}

#[test]
fn test_bridge_control_equality() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let a = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(1),
        control_kind: IngressControlKind::RefreshSnapshot,
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 0,
        legality_gate_id: LegalityGateId(1),
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_bridge_control_inequality() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let a = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(1),
        control_kind: IngressControlKind::RefreshSnapshot,
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 0,
        legality_gate_id: LegalityGateId(1),
    };
    let b = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(2),
        control_kind: IngressControlKind::RefreshSnapshot,
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 0,
        legality_gate_id: LegalityGateId(1),
    };
    assert_ne!(a, b);
}

#[test]
fn test_bridge_control_different_order_keys() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let a = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(1),
        control_kind: IngressControlKind::SetLabel {
            label: "a".to_string(),
        },
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 1,
        legality_gate_id: LegalityGateId(1),
    };
    let b = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(1),
        control_kind: IngressControlKind::SetLabel {
            label: "a".to_string(),
        },
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 2,
        legality_gate_id: LegalityGateId(1),
    };
    // BridgeControl doesn't implement PartialOrd, but we can verify the order keys differ
    assert_ne!(a.submission_order_key, b.submission_order_key);
    assert_eq!(a.submission_order_key, 1);
    assert_eq!(b.submission_order_key, 2);
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_bridge_control_zero_order_key() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(0),
        control_kind: IngressControlKind::RestoreObject,
        target_object_handle: None,
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: 0,
        legality_gate_id: LegalityGateId(0),
    };
    assert_eq!(control.submission_order_key, 0);
}

#[test]
fn test_bridge_control_max_order_key() {
    let session = SessionHandle::new(1);
    let runtime = RuntimeHandle::new(2);
    let control = BridgeControl {
        ingress_control_envelope_id: IngressControlEnvelopeId(u64::MAX),
        control_kind: IngressControlKind::RetireObject,
        target_object_handle: Some(ObjectHandle::new(u64::MAX)),
        target_runtime_handle: runtime,
        source_session_handle: session,
        submission_order_key: u64::MAX,
        legality_gate_id: LegalityGateId(u64::MAX),
    };
    assert_eq!(control.submission_order_key, u64::MAX);
}
