use engine_handle_refs::*;

// ============================================================================
// SessionHandle Tests
// ============================================================================

#[test]
fn test_session_handle_new() {
    let handle = SessionHandle::new(42);
    assert_eq!(handle.raw(), 42);
}

#[test]
fn test_session_handle_equality() {
    let a = SessionHandle::new(1);
    let b = SessionHandle::new(1);
    let c = SessionHandle::new(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_session_handle_opaque_tag_deterministic() {
    let handle = SessionHandle::new(123);
    let tag1 = handle.opaque_tag();
    let tag2 = handle.opaque_tag();
    assert_eq!(tag1, tag2);
    assert_eq!(tag1.len(), 16); // hex string
}

#[test]
fn test_session_handle_different_raw_values() {
    let h1 = SessionHandle::new(0);
    let h2 = SessionHandle::new(u64::MAX);
    assert_ne!(h1.raw(), h2.raw());
    assert_ne!(h1.opaque_tag(), h2.opaque_tag());
}

#[test]
fn test_session_handle_ordering() {
    let a = SessionHandle::new(1);
    let b = SessionHandle::new(2);
    assert!(a < b);
}

#[test]
fn test_session_handle_record() {
    let handle = SessionHandle::new(10);
    let record = SessionHandleRecord {
        session_handle: handle,
        session_scope_id: 100,
        origin_class: SessionOriginClass::Transport,
        status: SessionHandleStatus::Open,
        issued_at_tick: 42,
    };
    assert_eq!(record.session_handle.raw(), 10);
    assert_eq!(record.session_scope_id, 100);
    assert!(matches!(record.origin_class, SessionOriginClass::Transport));
    assert!(matches!(record.status, SessionHandleStatus::Open));
    assert_eq!(record.issued_at_tick, 42);
}

#[test]
fn test_session_origin_class_variants() {
    let transport = SessionOriginClass::Transport;
    let local = SessionOriginClass::LocalAttach;
    assert_ne!(transport as u8, local as u8);
}

#[test]
fn test_session_handle_status_variants() {
    assert_eq!(SessionHandleStatus::Open, SessionHandleStatus::Open);
    assert_ne!(SessionHandleStatus::Open, SessionHandleStatus::Closed);
    assert_ne!(SessionHandleStatus::Draining, SessionHandleStatus::Closed);
}

// ============================================================================
// ObjectHandle Tests
// ============================================================================

#[test]
fn test_object_handle_new() {
    let handle = ObjectHandle::new(99);
    assert_eq!(handle.raw(), 99);
}

#[test]
fn test_object_handle_equality() {
    let a = ObjectHandle::new(5);
    let b = ObjectHandle::new(5);
    assert_eq!(a, b);
}

#[test]
fn test_object_handle_opaque_tag() {
    let handle = ObjectHandle::new(456);
    let tag = handle.opaque_tag();
    assert_eq!(tag.len(), 16);
}

#[test]
fn test_object_handle_record() {
    let obj = ObjectHandle::new(1);
    let session = SessionHandle::new(2);
    let identity = IdentityRef {
        identity_class: IdentityClass::Object,
        tag: "obj-1".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::SessionLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: Some(session),
    };
    let record = ObjectHandleRecord {
        object_handle: obj,
        owner_session_handle: session,
        identity_ref: identity,
    };
    assert_eq!(record.object_handle.raw(), 1);
    assert_eq!(record.owner_session_handle.raw(), 2);
}

// ============================================================================
// RuntimeHandle Tests
// ============================================================================

#[test]
fn test_runtime_handle_new() {
    let handle = RuntimeHandle::new(77);
    assert_eq!(handle.raw(), 77);
}

#[test]
fn test_runtime_handle_equality() {
    let a = RuntimeHandle::new(10);
    let b = RuntimeHandle::new(10);
    let c = RuntimeHandle::new(11);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_runtime_handle_opaque_tag() {
    let handle = RuntimeHandle::new(789);
    let tag = handle.opaque_tag();
    assert_eq!(tag.len(), 16);
}

#[test]
fn test_runtime_handle_record() {
    let rt = RuntimeHandle::new(5);
    let session = SessionHandle::new(3);
    let record = RuntimeHandleRecord {
        runtime_handle: rt,
        owner_session_handle: session,
        issued_at_tick: 100,
    };
    assert_eq!(record.runtime_handle.raw(), 5);
    assert_eq!(record.issued_at_tick, 100);
}

// ============================================================================
// IdentityRef Tests
// ============================================================================

#[test]
fn test_identity_class_variants() {
    assert_ne!(IdentityClass::Session as u8, IdentityClass::Object as u8);
    assert_ne!(IdentityClass::Object as u8, IdentityClass::Runtime as u8);
}

#[test]
fn test_identity_visibility_scope_variants() {
    assert_ne!(
        IdentityVisibilityScope::SessionLocal as u8,
        IdentityVisibilityScope::Public as u8
    );
}

#[test]
fn test_identity_ref_status_variants() {
    assert_ne!(
        IdentityRefStatus::Active as u8,
        IdentityRefStatus::Revoked as u8
    );
}

#[test]
fn test_identity_ref_creation() {
    let identity = IdentityRef {
        identity_class: IdentityClass::Session,
        tag: "sess-001".to_string(),
        external_name: Some("main-session".to_string()),
        visibility_scope: IdentityVisibilityScope::Public,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    assert!(matches!(identity.identity_class, IdentityClass::Session));
    assert_eq!(identity.tag, "sess-001");
    assert_eq!(identity.external_name.as_deref(), Some("main-session"));
}

#[test]
fn test_identity_ref_equality() {
    let a = IdentityRef {
        identity_class: IdentityClass::Object,
        tag: "obj".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::RuntimeLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_identity_ref_inequality() {
    let a = IdentityRef {
        identity_class: IdentityClass::Object,
        tag: "obj-a".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::SessionLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    let b = IdentityRef {
        identity_class: IdentityClass::Object,
        tag: "obj-b".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::SessionLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    assert_ne!(a, b);
}

// ============================================================================
// StateRef Tests
// ============================================================================

#[test]
fn test_state_class_variants() {
    assert_ne!(StateClass::Snapshot as u8, StateClass::Selection as u8);
    assert_ne!(StateClass::Layout as u8, StateClass::Diagnostics as u8);
    assert_ne!(StateClass::Build as u8, StateClass::Snapshot as u8);
}

#[test]
fn test_state_retention_class_variants() {
    assert_ne!(
        StateRetentionClass::Ephemeral as u8,
        StateRetentionClass::SnapshotScoped as u8
    );
    assert_ne!(
        StateRetentionClass::CursorScoped as u8,
        StateRetentionClass::SnapshotScoped as u8
    );
}

#[test]
fn test_state_ref_creation() {
    let owner = IdentityRef {
        identity_class: IdentityClass::Runtime,
        tag: "rt-owner".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::RuntimeLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    let rt = RuntimeHandle::new(100);
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 42,
        fact_class_set: vec![StateClass::Snapshot, StateClass::Selection],
        retention_class: StateRetentionClass::SnapshotScoped,
    };
    assert!(matches!(state_ref.state_class, StateClass::Snapshot));
    assert_eq!(state_ref.snapshot_epoch, 42);
    assert_eq!(state_ref.fact_class_set.len(), 2);
}

#[test]
fn test_state_ref_equality() {
    let owner = IdentityRef {
        identity_class: IdentityClass::Session,
        tag: "owner".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::SessionLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    let rt = RuntimeHandle::new(1);
    let a = StateRef {
        state_class: StateClass::Layout,
        owner: owner.clone(),
        runtime_handle: rt,
        snapshot_epoch: 1,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

// ============================================================================
// ArtifactRef Tests
// ============================================================================

#[test]
fn test_artifact_kind_variants() {
    let kinds = [
        ArtifactKind::Preview,
        ArtifactKind::Validation,
        ArtifactKind::Build,
        ArtifactKind::Release,
        ArtifactKind::Evidence,
    ];
    // All distinct
    for i in 0..kinds.len() {
        for j in (i + 1)..kinds.len() {
            assert_ne!(kinds[i], kinds[j]);
        }
    }
}

#[test]
fn test_artifact_retention_policy_variants() {
    assert_ne!(
        ArtifactRetentionPolicy::Ephemeral as u8,
        ArtifactRetentionPolicy::Persistent as u8
    );
}

#[test]
fn test_artifact_ref_creation() {
    let rt = RuntimeHandle::new(10);
    let artifact = ArtifactRef {
        artifact_kind: ArtifactKind::Build,
        source_runtime_handle: rt,
        source_state_ref: None,
        content_digest: "sha256:abc123".to_string(),
        retention_policy: ArtifactRetentionPolicy::Persistent,
    };
    assert!(matches!(artifact.artifact_kind, ArtifactKind::Build));
    assert_eq!(artifact.content_digest, "sha256:abc123");
}

#[test]
fn test_artifact_ref_with_state() {
    let owner = IdentityRef {
        identity_class: IdentityClass::Runtime,
        tag: "owner".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::RuntimeLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    };
    let rt = RuntimeHandle::new(5);
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let artifact = ArtifactRef {
        artifact_kind: ArtifactKind::Evidence,
        source_runtime_handle: rt,
        source_state_ref: Some(state_ref),
        content_digest: "digest".to_string(),
        retention_policy: ArtifactRetentionPolicy::SnapshotScoped,
    };
    assert!(artifact.source_state_ref.is_some());
}

#[test]
fn test_artifact_ref_equality() {
    let rt = RuntimeHandle::new(1);
    let a = ArtifactRef {
        artifact_kind: ArtifactKind::Preview,
        source_runtime_handle: rt,
        source_state_ref: None,
        content_digest: "abc".to_string(),
        retention_policy: ArtifactRetentionPolicy::Ephemeral,
    };
    let b = a.clone();
    assert_eq!(a, b);
}

// ============================================================================
// Handle Ordering and Hash Tests
// ============================================================================

#[test]
fn test_session_handle_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let handle = SessionHandle::new(42);
    let mut hasher1 = DefaultHasher::new();
    handle.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    handle.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_object_handle_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let handle = ObjectHandle::new(99);
    let mut hasher1 = DefaultHasher::new();
    handle.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    handle.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_runtime_handle_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let handle = RuntimeHandle::new(7);
    let mut hasher1 = DefaultHasher::new();
    handle.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    handle.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_identity_class_ordering() {
    assert!(IdentityClass::Session < IdentityClass::Object);
    assert!(IdentityClass::Object < IdentityClass::Runtime);
}

#[test]
fn test_artifact_kind_ordering() {
    assert!(ArtifactKind::Preview < ArtifactKind::Validation);
    assert!(ArtifactKind::Build < ArtifactKind::Release);
}

#[test]
fn test_state_class_ordering() {
    assert!(StateClass::Snapshot < StateClass::Selection);
    assert!(StateClass::Build > StateClass::Diagnostics);
}
