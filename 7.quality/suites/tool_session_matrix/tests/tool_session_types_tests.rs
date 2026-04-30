// Tests for l6.0-tool-session: types, errors, publications, transactions

use stratumx_tooling_l6_0_tool_session::common::publication::PublicationChange;
use stratumx_tooling_l6_0_tool_session::*;

// ============================================================================
// ObjectHandle tests
// ============================================================================

#[test]
fn object_handle_constructs() {
    let h = ObjectHandle(42);
    assert_eq!(h.0, 42);
}

#[test]
fn object_handle_default_is_zero() {
    let h = ObjectHandle::default();
    assert_eq!(h.0, 0);
}

#[test]
fn object_handle_equality() {
    assert_eq!(ObjectHandle(1), ObjectHandle(1));
    assert_ne!(ObjectHandle(1), ObjectHandle(2));
}

#[test]
fn object_handle_ordering() {
    assert!(ObjectHandle(1) < ObjectHandle(2));
    assert!(ObjectHandle(5) > ObjectHandle(3));
}

#[test]
fn object_handle_serializes() {
    let h = ObjectHandle(123);
    let json = serde_json::to_string(&h).unwrap();
    assert_eq!(json, "123");
}

#[test]
fn object_handle_deserializes() {
    let h: ObjectHandle = serde_json::from_str("456").unwrap();
    assert_eq!(h.0, 456);
}

#[test]
fn object_handle_is_hashable() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(ObjectHandle(1));
    set.insert(ObjectHandle(2));
    set.insert(ObjectHandle(1));
    assert_eq!(set.len(), 2);
}

// ============================================================================
// ObjectClass tests
// ============================================================================

#[test]
fn object_class_variants() {
    let classes = [
        ObjectClass::World,
        ObjectClass::Scene,
        ObjectClass::Terrain,
        ObjectClass::Material,
        ObjectClass::Logic,
        ObjectClass::Asset,
        ObjectClass::Build,
    ];
    assert_eq!(classes.len(), 7);
}

#[test]
fn object_class_equality() {
    assert_eq!(ObjectClass::World, ObjectClass::World);
    assert_ne!(ObjectClass::World, ObjectClass::Scene);
}

#[test]
fn object_class_serializes() {
    let json = serde_json::to_string(&ObjectClass::Material).unwrap();
    assert!(json.contains("Material"));
}

#[test]
fn object_class_deserializes() {
    let c: ObjectClass = serde_json::from_str("\"Terrain\"").unwrap();
    assert_eq!(c, ObjectClass::Terrain);
}

#[test]
fn object_class_ordering() {
    // ObjectClass derives Ord - just verify comparison works
    let _ = ObjectClass::World < ObjectClass::Scene;
    let _ = ObjectClass::World == ObjectClass::World;
    // All variants should be comparable
    let _ = ObjectClass::Terrain <= ObjectClass::Material;
}

// ============================================================================
// ToolingError tests
// ============================================================================

#[test]
fn tooling_error_unknown_object() {
    let e = ToolingError::UnknownObject;
    assert_eq!(format!("{}", e), "unknown object");
}

#[test]
fn tooling_error_no_build_artifact() {
    let e = ToolingError::NoBuildArtifact;
    assert_eq!(format!("{}", e), "no build artifact");
}

#[test]
fn tooling_error_unsupported() {
    let e = ToolingError::Unsupported;
    assert_eq!(format!("{}", e), "unsupported operation");
}

#[test]
fn tooling_error_message() {
    let e = ToolingError::Message("custom error".into());
    assert_eq!(format!("{}", e), "custom error");
}

#[test]
fn tooling_error_precondition_failed() {
    let e = ToolingError::PreconditionFailed(DisabledReason::NoWorldOpen);
    let msg = format!("{}", e);
    assert!(msg.contains("precondition failed"));
}

#[test]
fn tooling_error_from_string() {
    let e: ToolingError = String::from("error text").into();
    assert_eq!(format!("{}", e), "error text");
}

#[test]
fn tooling_error_equality() {
    assert_eq!(ToolingError::UnknownObject, ToolingError::UnknownObject);
    assert_eq!(
        ToolingError::Message("x".into()),
        ToolingError::Message("x".into())
    );
    assert_ne!(ToolingError::UnknownObject, ToolingError::Unsupported);
}

#[test]
fn tooling_error_is_std_error() {
    fn assert_error<T: std::error::Error>() {}
    assert_error::<ToolingError>();
}

#[test]
fn tooling_error_display() {
    let e = ToolingError::Message("test".into());
    let s = e.to_string();
    assert_eq!(s, "test");
}

// ============================================================================
// DisabledReason tests
// ============================================================================

#[test]
fn disabled_reason_variants() {
    let reasons = [
        DisabledReason::NoWorldOpen,
        DisabledReason::NoProjectOpen,
        DisabledReason::MaterialAuthorityUnavailable,
        DisabledReason::AudioAuthorityUnavailable,
        DisabledReason::RuntimeKernelUnavailable,
        DisabledReason::TerrainNotAvailable,
    ];
    assert_eq!(reasons.len(), 6);
}

#[test]
fn disabled_reason_invalid_input() {
    let r = DisabledReason::InvalidInput("bad input".into());
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("InvalidInput"));
}

#[test]
fn disabled_reason_serializes() {
    let r = DisabledReason::NoWorldOpen;
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("NoWorldOpen"));
}

#[test]
fn disabled_reason_deserializes() {
    let r: DisabledReason = serde_json::from_str("\"NoProjectOpen\"").unwrap();
    assert_eq!(r, DisabledReason::NoProjectOpen);
}

#[test]
fn disabled_reason_equality() {
    assert_eq!(DisabledReason::NoWorldOpen, DisabledReason::NoWorldOpen);
    assert_ne!(DisabledReason::NoWorldOpen, DisabledReason::NoProjectOpen);
}

// ============================================================================
// ToolObject tests
// ============================================================================

#[test]
fn tool_object_constructs() {
    let obj = ToolObject {
        handle: ObjectHandle(1),
        label: "TestObject".into(),
        class: ObjectClass::World,
        active: true,
        fields: Default::default(),
        tags: Default::default(),
    };
    assert_eq!(obj.label, "TestObject");
    assert!(obj.active);
}

#[test]
fn tool_object_serializes() {
    let obj = ToolObject {
        handle: ObjectHandle(42),
        label: "SerObj".into(),
        class: ObjectClass::Material,
        active: false,
        fields: Default::default(),
        tags: Default::default(),
    };
    let json = serde_json::to_string(&obj).unwrap();
    assert!(json.contains("SerObj"));
    assert!(json.contains("Material"));
}

#[test]
fn tool_object_deserializes() {
    let json = r#"{
        "handle": 99,
        "label": "DeserObj",
        "class": "Terrain",
        "active": true,
        "fields": {},
        "tags": []
    }"#;
    let obj: ToolObject = serde_json::from_str(json).unwrap();
    assert_eq!(obj.handle.0, 99);
    assert_eq!(obj.label, "DeserObj");
    assert_eq!(obj.class, ObjectClass::Terrain);
}

#[test]
fn tool_object_roundtrip() {
    let obj = ToolObject {
        handle: ObjectHandle(1),
        label: "RoundTrip".into(),
        class: ObjectClass::Build,
        active: true,
        fields: Default::default(),
        tags: Default::default(),
    };
    let json = serde_json::to_string(&obj).unwrap();
    let restored: ToolObject = serde_json::from_str(&json).unwrap();
    assert_eq!(obj, restored);
}

#[test]
fn tool_object_with_fields() {
    use std::collections::BTreeMap;
    let mut fields = BTreeMap::new();
    fields.insert("key".into(), "value".into());

    let obj = ToolObject {
        handle: ObjectHandle(1),
        label: "Fielded".into(),
        class: ObjectClass::Asset,
        active: true,
        fields,
        tags: Default::default(),
    };
    assert_eq!(obj.fields.get("key"), Some(&"value".to_string()));
}

#[test]
fn tool_object_with_tags() {
    use std::collections::BTreeSet;
    let mut tags = BTreeSet::new();
    tags.insert("test".into());
    tags.insert("integration".into());

    let obj = ToolObject {
        handle: ObjectHandle(1),
        label: "Tagged".into(),
        class: ObjectClass::Asset,
        active: true,
        fields: Default::default(),
        tags,
    };
    assert_eq!(obj.tags.len(), 2);
    assert!(obj.tags.contains("test"));
}

// ============================================================================
// ToolSnapshot tests
// ============================================================================

#[test]
fn tool_snapshot_default_empty() {
    let snap = ToolSnapshot::default();
    assert!(snap.objects.is_empty());
}

#[test]
fn tool_snapshot_serializes() {
    let snap = ToolSnapshot { objects: vec![] };
    let json = serde_json::to_string(&snap).unwrap();
    assert!(json.contains("objects"));
}

#[test]
fn tool_snapshot_with_objects() {
    let obj = ToolObject {
        handle: ObjectHandle(1),
        label: "SnapObj".into(),
        class: ObjectClass::World,
        active: true,
        fields: Default::default(),
        tags: Default::default(),
    };
    let snap = ToolSnapshot { objects: vec![obj] };
    assert_eq!(snap.objects.len(), 1);
}

// ============================================================================
// CommandOrigin, ApprovalClass, BudgetClass tests
// ============================================================================

#[test]
fn command_origin_variants() {
    assert_eq!(CommandOrigin::User, CommandOrigin::User);
    assert_ne!(CommandOrigin::User, CommandOrigin::Automation);
}

#[test]
fn command_origin_serializes() {
    let json = serde_json::to_string(&CommandOrigin::Automation).unwrap();
    assert!(json.contains("Automation"));
}

#[test]
fn approval_class_variants() {
    assert_eq!(ApprovalClass::None, ApprovalClass::None);
    assert_ne!(ApprovalClass::None, ApprovalClass::ReviewRequired);
}

#[test]
fn approval_class_serializes() {
    let json = serde_json::to_string(&ApprovalClass::ReviewRequired).unwrap();
    assert!(json.contains("ReviewRequired"));
}

#[test]
fn budget_class_variants() {
    assert_eq!(BudgetClass::Interactive, BudgetClass::Interactive);
    assert_ne!(BudgetClass::Interactive, BudgetClass::Background);
}

#[test]
fn budget_class_serializes() {
    let json = serde_json::to_string(&BudgetClass::Background).unwrap();
    assert!(json.contains("Background"));
}

// ============================================================================
// Build/Release artifact tests
// ============================================================================

#[test]
fn build_manifest_constructs() {
    let m = BuildManifest {
        digest: "abc123".into(),
    };
    assert_eq!(m.digest, "abc123");
}

#[test]
fn build_manifest_serializes() {
    let m = BuildManifest {
        digest: "sha256".into(),
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("sha256"));
}

#[test]
fn build_artifact_constructs() {
    let a = BuildArtifact {
        digest: "d1".into(),
        object_count: 42,
        manifest: BuildManifest {
            digest: "m1".into(),
        },
    };
    assert_eq!(a.object_count, 42);
}

#[test]
fn build_artifact_serializes() {
    let a = BuildArtifact {
        digest: "bd1".into(),
        object_count: 10,
        manifest: BuildManifest {
            digest: "md1".into(),
        },
    };
    let json = serde_json::to_string(&a).unwrap();
    assert!(json.contains("bd1"));
    assert!(json.contains("10"));
}

#[test]
fn release_manifest_constructs() {
    let m = ReleaseManifest {
        digest: "rd1".into(),
    };
    assert_eq!(m.digest, "rd1");
}

#[test]
fn release_package_constructs() {
    let p = ReleasePackage {
        build_digest: "bd".into(),
        channel: "stable".into(),
        signed: true,
        manifest: ReleaseManifest {
            digest: "rm".into(),
        },
    };
    assert!(p.signed);
    assert_eq!(p.channel, "stable");
}

#[test]
fn release_package_serializes() {
    let p = ReleasePackage {
        build_digest: "bd".into(),
        channel: "beta".into(),
        signed: false,
        manifest: ReleaseManifest {
            digest: "rm".into(),
        },
    };
    let json = serde_json::to_string(&p).unwrap();
    assert!(json.contains("beta"));
    assert!(json.contains("false"));
}

// ============================================================================
// Preview types tests
// ============================================================================

#[test]
fn preview_result_constructs() {
    let r = PreviewResult {
        handle: ObjectHandle(1),
        summary: "Preview OK".into(),
    };
    assert_eq!(r.summary, "Preview OK");
}

#[test]
fn preview_mode_variants() {
    assert_ne!(PreviewMode::Isolated, PreviewMode::Contextual);
}

#[test]
fn preview_mode_serializes() {
    let json = serde_json::to_string(&PreviewMode::Contextual).unwrap();
    assert!(json.contains("Contextual"));
}

#[test]
fn preview_session_constructs() {
    let s = PreviewSession {
        session_id: "sess1".into(),
        source_handle: ObjectHandle(5),
        mode: PreviewMode::Isolated,
        active: true,
    };
    assert!(s.active);
    assert_eq!(s.session_id, "sess1");
}

#[test]
fn preview_session_serializes() {
    let s = PreviewSession {
        session_id: "sess1".into(),
        source_handle: ObjectHandle(5),
        mode: PreviewMode::Contextual,
        active: false,
    };
    let json = serde_json::to_string(&s).unwrap();
    assert!(json.contains("sess1"));
    assert!(json.contains("Contextual"));
}

// ============================================================================
// Mutation tests
// ============================================================================

#[test]
fn mutation_object_created() {
    let m = Mutation::ObjectCreated {
        handle: ObjectHandle(1),
        class: ObjectClass::World,
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("ObjectCreated"));
}

#[test]
fn mutation_object_deleted() {
    let m = Mutation::ObjectDeleted {
        handle: ObjectHandle(2),
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("ObjectDeleted"));
}

#[test]
fn mutation_field_set() {
    let m = Mutation::FieldSet {
        handle: ObjectHandle(3),
        field: "name".into(),
        value: Some("test".into()),
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("FieldSet"));
    assert!(json.contains("name"));
}

#[test]
fn mutation_field_set_none() {
    let m = Mutation::FieldSet {
        handle: ObjectHandle(3),
        field: "name".into(),
        value: None,
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("null"));
}

#[test]
fn mutation_tag_added() {
    let m = Mutation::TagAdded {
        handle: ObjectHandle(4),
        tag: "renderable".into(),
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("TagAdded"));
}

#[test]
fn mutation_tag_removed() {
    let m = Mutation::TagRemoved {
        handle: ObjectHandle(4),
        tag: "old".into(),
    };
    let json = serde_json::to_string(&m).unwrap();
    assert!(json.contains("TagRemoved"));
}

#[test]
fn mutation_roundtrip() {
    let m = Mutation::ObjectCreated {
        handle: ObjectHandle(10),
        class: ObjectClass::Material,
    };
    let json = serde_json::to_string(&m).unwrap();
    let restored: Mutation = serde_json::from_str(&json).unwrap();
    assert_eq!(m, restored);
}

// ============================================================================
// Rollback tests
// ============================================================================

#[test]
fn rollback_operation_delete() {
    let op = RollbackOperation::DeleteObject {
        handle: ObjectHandle(1),
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("DeleteObject"));
}

#[test]
fn rollback_operation_restore() {
    let obj = ToolObject {
        handle: ObjectHandle(1),
        label: "Restored".into(),
        class: ObjectClass::World,
        active: true,
        fields: Default::default(),
        tags: Default::default(),
    };
    let op = RollbackOperation::RestoreObject {
        handle: ObjectHandle(1),
        object: obj,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("RestoreObject"));
}

#[test]
fn rollback_operation_restore_field() {
    let op = RollbackOperation::RestoreField {
        handle: ObjectHandle(1),
        field: "color".into(),
        value: Some("red".into()),
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("RestoreField"));
}

#[test]
fn rollback_operation_remove_tag() {
    let op = RollbackOperation::RemoveTag {
        handle: ObjectHandle(1),
        tag: "temp".into(),
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("RemoveTag"));
}

#[test]
fn rollback_operation_add_tag() {
    let op = RollbackOperation::AddTag {
        handle: ObjectHandle(1),
        tag: "final".into(),
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("AddTag"));
}

#[test]
fn rollback_binding_constructs() {
    let rb = RollbackBinding {
        operations: vec![RollbackOperation::DeleteObject {
            handle: ObjectHandle(1),
        }],
    };
    assert_eq!(rb.operations.len(), 1);
}

#[test]
fn rollback_binding_serializes() {
    let rb = RollbackBinding {
        operations: vec![
            RollbackOperation::AddTag {
                handle: ObjectHandle(1),
                tag: "tag1".into(),
            },
            RollbackOperation::RemoveTag {
                handle: ObjectHandle(1),
                tag: "tag1".into(),
            },
        ],
    };
    let json = serde_json::to_string(&rb).unwrap();
    let restored: RollbackBinding = serde_json::from_str(&json).unwrap();
    assert_eq!(rb, restored);
}

// ============================================================================
// ToolTransaction tests
// ============================================================================

#[test]
fn tool_transaction_constructs() {
    let t = ToolTransaction {
        order: 1,
        origin: CommandOrigin::User,
        budget: BudgetClass::Interactive,
        summary: "Test transaction".into(),
        mutation_set: vec![],
        rollback_binding: RollbackBinding { operations: vec![] },
        committed: false,
    };
    assert_eq!(t.order, 1);
    assert!(!t.committed);
}

#[test]
fn tool_transaction_with_mutations() {
    let t = ToolTransaction {
        order: 1,
        origin: CommandOrigin::Automation,
        budget: BudgetClass::Background,
        summary: "Create and tag".into(),
        mutation_set: vec![
            Mutation::ObjectCreated {
                handle: ObjectHandle(1),
                class: ObjectClass::Asset,
            },
            Mutation::TagAdded {
                handle: ObjectHandle(1),
                tag: "new".into(),
            },
        ],
        rollback_binding: RollbackBinding {
            operations: vec![RollbackOperation::DeleteObject {
                handle: ObjectHandle(1),
            }],
        },
        committed: true,
    };
    assert_eq!(t.mutation_set.len(), 2);
}

#[test]
fn tool_transaction_serializes() {
    let t = ToolTransaction {
        order: 5,
        origin: CommandOrigin::User,
        budget: BudgetClass::Interactive,
        summary: "tx_serialize".into(),
        mutation_set: vec![],
        rollback_binding: RollbackBinding { operations: vec![] },
        committed: false,
    };
    let json = serde_json::to_string(&t).unwrap();
    assert!(json.contains("tx_serialize"));
    assert!(json.contains("User"));
}

#[test]
fn tool_transaction_roundtrip() {
    let t = ToolTransaction {
        order: 42,
        origin: CommandOrigin::Automation,
        budget: BudgetClass::Background,
        summary: "Roundtrip".into(),
        mutation_set: vec![Mutation::FieldSet {
            handle: ObjectHandle(1),
            field: "x".into(),
            value: Some("1".into()),
        }],
        rollback_binding: RollbackBinding {
            operations: vec![RollbackOperation::RestoreField {
                handle: ObjectHandle(1),
                field: "x".into(),
                value: Some("0".into()),
            }],
        },
        committed: true,
    };
    let json = serde_json::to_string(&t).unwrap();
    let restored: ToolTransaction = serde_json::from_str(&json).unwrap();
    assert_eq!(t, restored);
}

// ============================================================================
// BranchCoverage tests
// ============================================================================

#[test]
fn branch_coverage_all_complete() {
    let bc = BranchCoverage {
        physical_complete: true,
        visual_complete: true,
        acoustic_complete: true,
        light_complete: true,
        runtime_complete: true,
        missing_bindings: vec![],
        invalid_combinations: vec![],
    };
    assert!(bc.missing_bindings.is_empty());
    assert!(bc.invalid_combinations.is_empty());
}

#[test]
fn branch_coverage_with_missing() {
    let bc = BranchCoverage {
        physical_complete: true,
        visual_complete: false,
        acoustic_complete: false,
        light_complete: true,
        runtime_complete: true,
        missing_bindings: vec!["visual".into(), "acoustic".into()],
        invalid_combinations: vec![],
    };
    assert_eq!(bc.missing_bindings.len(), 2);
}

#[test]
fn branch_coverage_serializes() {
    let bc = BranchCoverage {
        physical_complete: true,
        visual_complete: true,
        acoustic_complete: false,
        light_complete: true,
        runtime_complete: false,
        missing_bindings: vec!["acoustic".into()],
        invalid_combinations: vec!["light+runtime conflict".into()],
    };
    let json = serde_json::to_string(&bc).unwrap();
    assert!(json.contains("acoustic"));
}

#[test]
fn branch_coverage_roundtrip() {
    let bc = BranchCoverage {
        physical_complete: false,
        visual_complete: true,
        acoustic_complete: true,
        light_complete: false,
        runtime_complete: true,
        missing_bindings: vec!["physical".into(), "light".into()],
        invalid_combinations: vec![],
    };
    let json = serde_json::to_string(&bc).unwrap();
    let restored: BranchCoverage = serde_json::from_str(&json).unwrap();
    assert_eq!(bc, restored);
}

// ============================================================================
// PublicationChange tests
// ============================================================================

#[test]
fn publication_change_constructs() {
    let pc = PublicationChange {
        button_id: "btn.test".into(),
        owner_state: "state1".into(),
    };
    assert_eq!(pc.button_id, "btn.test");
}

#[test]
fn publication_change_serializes() {
    let pc = PublicationChange {
        button_id: "btn.material.create".into(),
        owner_state: "created".into(),
    };
    let json = serde_json::to_string(&pc).unwrap();
    assert!(json.contains("btn.material.create"));
}

#[test]
fn publication_change_roundtrip() {
    let pc = PublicationChange {
        button_id: "btn.world.open".into(),
        owner_state: "open".into(),
    };
    let json = serde_json::to_string(&pc).unwrap();
    let restored: PublicationChange = serde_json::from_str(&json).unwrap();
    assert_eq!(pc, restored);
}

// ============================================================================
// EditorPublication tests
// ============================================================================

#[test]
fn editor_publication_variants() {
    let change = PublicationChange {
        button_id: "btn.test".into(),
        owner_state: "s1".into(),
    };
    let pubs_to_test = [
        EditorPublication::ShellChanged(change.clone()),
        EditorPublication::ProjectChanged(change.clone()),
        EditorPublication::WorldChanged(change.clone()),
        EditorPublication::TerrainChanged(change.clone()),
        EditorPublication::MaterialChanged(change.clone()),
        EditorPublication::EnvironmentChanged(change.clone()),
        EditorPublication::AudioChanged(change.clone()),
        EditorPublication::RuntimeChanged(change.clone()),
        EditorPublication::ValidationChanged(change.clone()),
        EditorPublication::DiagnosticsChanged(change.clone()),
        EditorPublication::EvidenceChanged(change.clone()),
        EditorPublication::BuildChanged(change),
    ];
    assert_eq!(pubs_to_test.len(), 12);
}

#[test]
fn editor_publication_serializes() {
    let change = PublicationChange {
        button_id: "btn.material.bind".into(),
        owner_state: "bound".into(),
    };
    let pub_ = EditorPublication::MaterialChanged(change);
    let json = serde_json::to_string(&pub_).unwrap();
    assert!(json.contains("MaterialChanged"));
}

#[test]
fn editor_publication_roundtrip() {
    let change = PublicationChange {
        button_id: "btn.audio.play".into(),
        owner_state: "playing".into(),
    };
    let pub_ = EditorPublication::AudioChanged(change);
    let json = serde_json::to_string(&pub_).unwrap();
    let restored: EditorPublication = serde_json::from_str(&json).unwrap();
    assert_eq!(pub_, restored);
}

// ============================================================================
// ValidationDiagnostic tests
// ============================================================================

#[test]
fn validation_diagnostic_constructs() {
    let d = ValidationDiagnostic {
        message: "Validation failed".into(),
    };
    assert_eq!(d.message, "Validation failed");
}

#[test]
fn validation_diagnostic_serializes() {
    let d = ValidationDiagnostic {
        message: "Error".into(),
    };
    let json = serde_json::to_string(&d).unwrap();
    assert!(json.contains("Error"));
}

// ============================================================================
// PlannedGoal tests
// ============================================================================

#[test]
fn planned_goal_constructs() {
    let g = PlannedGoal {
        goal: "Create material".into(),
        suggested_commands: vec!["material.create".into()],
    };
    assert_eq!(g.goal, "Create material");
    assert_eq!(g.suggested_commands.len(), 1);
}

#[test]
fn planned_goal_serializes() {
    let g = PlannedGoal {
        goal: "Test".into(),
        suggested_commands: vec!["cmd1".into(), "cmd2".into()],
    };
    let json = serde_json::to_string(&g).unwrap();
    assert!(json.contains("Test"));
}

// ============================================================================
// L60ToolSessionMarker tests
// ============================================================================

#[test]
fn marker_default() {
    let m = L60ToolSessionMarker;
    assert_eq!(m, L60ToolSessionMarker);
}

#[test]
fn marker_copy() {
    let a = L60ToolSessionMarker;
    let _b = a;
    let _c = a;
}

#[test]
fn marker_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h1 = DefaultHasher::new();
    L60ToolSessionMarker.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    L60ToolSessionMarker.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// CANONICAL_LEVEL constant
// ============================================================================

#[test]
fn canonical_level_value() {
    assert_eq!(CANONICAL_LEVEL, "l6.0-tool-session");
}
