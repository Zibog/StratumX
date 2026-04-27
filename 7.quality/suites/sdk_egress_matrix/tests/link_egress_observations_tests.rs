use engine_handle_refs::{
    IdentityClass, IdentityRef, IdentityRefStatus, IdentityVisibilityScope, RuntimeHandle,
    StateClass, StateRef, StateRetentionClass,
};
use link_egress_observations::*;
use sdk_compat::CompatibilityProfile;

// ============================================================================
// Constants
// ============================================================================

#[test]
fn test_max_batch_records_constant() {
    assert_eq!(MAX_BATCH_RECORDS, 256);
}

// ============================================================================
// ObservationBatchId Tests
// ============================================================================

#[test]
fn test_observation_batch_id_new() {
    let id = ObservationBatchId(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_observation_batch_id_equality() {
    let a = ObservationBatchId(1);
    let b = ObservationBatchId(1);
    assert_eq!(a, b);
}

#[test]
fn test_observation_batch_id_ordering() {
    let a = ObservationBatchId(1);
    let b = ObservationBatchId(2);
    assert!(a < b);
}

#[test]
fn test_observation_batch_id_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = ObservationBatchId(100);
    let mut h1 = DefaultHasher::new();
    id.hash(&mut h1);
    let mut h2 = DefaultHasher::new();
    id.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

// ============================================================================
// ObservationKind Tests
// ============================================================================

#[test]
fn test_observation_kind_packet_accepted() {
    let kind = ObservationKind::PacketAccepted;
    assert!(matches!(kind, ObservationKind::PacketAccepted));
}

#[test]
fn test_observation_kind_control_applied() {
    let kind = ObservationKind::ControlApplied;
    assert!(matches!(kind, ObservationKind::ControlApplied));
}

#[test]
fn test_observation_kind_snapshot_published() {
    let kind = ObservationKind::SnapshotPublished;
    assert!(matches!(kind, ObservationKind::SnapshotPublished));
}

#[test]
fn test_observation_kind_object_retired() {
    let kind = ObservationKind::ObjectRetired;
    assert!(matches!(kind, ObservationKind::ObjectRetired));
}

#[test]
fn test_observation_kind_object_restored() {
    let kind = ObservationKind::ObjectRestored;
    assert!(matches!(kind, ObservationKind::ObjectRestored));
}

#[test]
fn test_observation_kind_equality() {
    let a = ObservationKind::PacketAccepted;
    let b = ObservationKind::PacketAccepted;
    assert_eq!(a, b);
}

#[test]
fn test_observation_kind_inequality() {
    let a = ObservationKind::PacketAccepted;
    let b = ObservationKind::ControlApplied;
    assert_ne!(a, b);
}

#[test]
fn test_observation_kind_all_variants_distinct() {
    let kinds = [
        ObservationKind::PacketAccepted,
        ObservationKind::ControlApplied,
        ObservationKind::SnapshotPublished,
        ObservationKind::ObjectRetired,
        ObservationKind::ObjectRestored,
    ];
    for i in 0..kinds.len() {
        for j in (i + 1)..kinds.len() {
            assert_ne!(kinds[i], kinds[j]);
        }
    }
}

// ============================================================================
// ObservationRecord Tests
// ============================================================================

#[test]
fn test_observation_record_creation() {
    let record = ObservationRecord {
        cursor: 1,
        kind: ObservationKind::PacketAccepted,
        summary: "Packet received and accepted".to_string(),
    };
    assert_eq!(record.cursor, 1);
    assert!(matches!(record.kind, ObservationKind::PacketAccepted));
}

#[test]
fn test_observation_record_equality() {
    let a = ObservationRecord {
        cursor: 1,
        kind: ObservationKind::ControlApplied,
        summary: "Control applied".to_string(),
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_observation_record_inequality() {
    let a = ObservationRecord {
        cursor: 1,
        kind: ObservationKind::ControlApplied,
        summary: "Control A".to_string(),
    };
    let b = ObservationRecord {
        cursor: 2,
        kind: ObservationKind::ControlApplied,
        summary: "Control A".to_string(),
    };
    assert_ne!(a, b);
}

#[test]
fn test_observation_record_empty_summary() {
    let record = ObservationRecord {
        cursor: 0,
        kind: ObservationKind::SnapshotPublished,
        summary: "".to_string(),
    };
    assert!(record.summary.is_empty());
}

// ============================================================================
// ObservationBatch Tests
// ============================================================================

fn make_identity_ref() -> IdentityRef {
    IdentityRef {
        identity_class: IdentityClass::Runtime,
        tag: "rt".to_string(),
        external_name: None,
        visibility_scope: IdentityVisibilityScope::RuntimeLocal,
        status: IdentityRefStatus::Active,
        source_session_handle: None,
    }
}

#[test]
fn test_observation_batch_creation() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let batch = ObservationBatch {
        observation_batch_id: ObservationBatchId(1),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![StateClass::Snapshot],
        publication_cursor: 0,
        emitted_at_tick: 1000,
        profile: CompatibilityProfile::ToolRuntime,
        records: vec![],
    };
    assert_eq!(batch.observation_batch_id, ObservationBatchId(1));
    assert!(batch.records.is_empty());
    assert_eq!(batch.emitted_at_tick, 1000);
}

#[test]
fn test_observation_batch_with_records() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let records = vec![
        ObservationRecord {
            cursor: 1,
            kind: ObservationKind::PacketAccepted,
            summary: "Accepted".to_string(),
        },
        ObservationRecord {
            cursor: 2,
            kind: ObservationKind::ControlApplied,
            summary: "Applied".to_string(),
        },
    ];
    let batch = ObservationBatch {
        observation_batch_id: ObservationBatchId(2),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![],
        publication_cursor: 2,
        emitted_at_tick: 2000,
        profile: CompatibilityProfile::EditorSurface,
        records,
    };
    assert_eq!(batch.records.len(), 2);
}

#[test]
fn test_observation_batch_equality() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let a = ObservationBatch {
        observation_batch_id: ObservationBatchId(1),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![],
        publication_cursor: 0,
        emitted_at_tick: 0,
        profile: CompatibilityProfile::ToolRuntime,
        records: vec![],
    };
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_observation_batch_all_profiles() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let profiles = [
        CompatibilityProfile::ToolRuntime,
        CompatibilityProfile::EditorSurface,
        CompatibilityProfile::Automation,
        CompatibilityProfile::Diagnostics,
    ];
    for profile in profiles {
        let batch = ObservationBatch {
            observation_batch_id: ObservationBatchId(1),
            source_runtime_handle: rt,
            source_state_ref: state_ref.clone(),
            fact_class_set: vec![],
            publication_cursor: 0,
            emitted_at_tick: 0,
            profile,
            records: vec![],
        };
        assert_eq!(batch.profile, profile);
    }
}

// ============================================================================
// batch_after Function Tests
// ============================================================================

#[test]
fn test_batch_after_filters_by_cursor() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let batch = ObservationBatch {
        observation_batch_id: ObservationBatchId(1),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![],
        publication_cursor: 3,
        emitted_at_tick: 100,
        profile: CompatibilityProfile::ToolRuntime,
        records: vec![
            ObservationRecord {
                cursor: 1,
                kind: ObservationKind::PacketAccepted,
                summary: "a".to_string(),
            },
            ObservationRecord {
                cursor: 2,
                kind: ObservationKind::ControlApplied,
                summary: "b".to_string(),
            },
            ObservationRecord {
                cursor: 3,
                kind: ObservationKind::SnapshotPublished,
                summary: "c".to_string(),
            },
        ],
    };

    let result = batch_after(&batch, 1, 10);
    assert_eq!(result.records.len(), 2);
    assert_eq!(result.records[0].cursor, 2);
    assert_eq!(result.records[1].cursor, 3);
}

#[test]
fn test_batch_after_empty_result() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let batch = ObservationBatch {
        observation_batch_id: ObservationBatchId(1),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![],
        publication_cursor: 1,
        emitted_at_tick: 100,
        profile: CompatibilityProfile::ToolRuntime,
        records: vec![ObservationRecord {
            cursor: 1,
            kind: ObservationKind::PacketAccepted,
            summary: "a".to_string(),
        }],
    };

    let result = batch_after(&batch, 10, 10);
    assert!(result.records.is_empty());
}

#[test]
fn test_batch_after_respects_max_records() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let records: Vec<ObservationRecord> = (1..=10)
        .map(|i| ObservationRecord {
            cursor: i as u64,
            kind: ObservationKind::PacketAccepted,
            summary: format!("record_{}", i),
        })
        .collect();

    let batch = ObservationBatch {
        observation_batch_id: ObservationBatchId(1),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![],
        publication_cursor: 10,
        emitted_at_tick: 100,
        profile: CompatibilityProfile::ToolRuntime,
        records,
    };

    let result = batch_after(&batch, 0, 3);
    assert_eq!(result.records.len(), 3);
}

#[test]
fn test_batch_after_preserves_metadata() {
    let rt = RuntimeHandle::new(1);
    let owner = make_identity_ref();
    let state_ref = StateRef {
        state_class: StateClass::Snapshot,
        owner,
        runtime_handle: rt,
        snapshot_epoch: 0,
        fact_class_set: vec![StateClass::Snapshot],
        retention_class: StateRetentionClass::Ephemeral,
    };
    let batch = ObservationBatch {
        observation_batch_id: ObservationBatchId(42),
        source_runtime_handle: rt,
        source_state_ref: state_ref,
        fact_class_set: vec![StateClass::Snapshot],
        publication_cursor: 5,
        emitted_at_tick: 5000,
        profile: CompatibilityProfile::Diagnostics,
        records: vec![ObservationRecord {
            cursor: 10,
            kind: ObservationKind::ObjectRestored,
            summary: "restored".to_string(),
        }],
    };

    let result = batch_after(&batch, 5, 10);
    assert_eq!(result.observation_batch_id, ObservationBatchId(42));
    assert_eq!(result.emitted_at_tick, 5000);
    assert_eq!(result.profile, CompatibilityProfile::Diagnostics);
}

// ============================================================================
// Scene DTO Serialization Tests (serde)
// ============================================================================

#[test]
fn test_scene_dto_serialize_deserialize() {
    let scene = SceneDto {
        scene_name: "test_scene".to_string(),
        terrain: TerrainPatchDto {
            entity_id: 1,
            origin: [0.0, 0.0, 0.0],
            world_size: [100.0, 100.0],
            resolution: [256, 256],
            mesh_revision: 1,
        },
        wall: WallDto {
            entity_id: 2,
            position: [10.0, 0.0, 10.0],
            dimensions: [5.0, 3.0, 0.5],
            stack_id: 1,
        },
        weapon: WeaponDto {
            entity_id: 3,
            profile_id: 1,
            position: [0.0, 1.5, 0.0],
            aim_direction: [0.0, 0.0, 1.0],
        },
        camera: CameraDto {
            position: [0.0, 5.0, -10.0],
            look_at: [0.0, 1.5, 0.0],
            fov_deg: 90.0,
        },
        sky_bundle_path: Some("sky/default".to_string()),
    };
    let json = serde_json::to_string(&scene).unwrap();
    let parsed: SceneDto = serde_json::from_str(&json).unwrap();
    assert_eq!(scene, parsed);
}

#[test]
fn test_scene_dto_null_sky_bundle() {
    let scene = SceneDto {
        scene_name: "empty".to_string(),
        terrain: TerrainPatchDto {
            entity_id: 0,
            origin: [0.0; 3],
            world_size: [10.0; 2],
            resolution: [64, 64],
            mesh_revision: 0,
        },
        wall: WallDto {
            entity_id: 0,
            position: [0.0; 3],
            dimensions: [0.0; 3],
            stack_id: 0,
        },
        weapon: WeaponDto {
            entity_id: 0,
            profile_id: 0,
            position: [0.0; 3],
            aim_direction: [0.0; 3],
        },
        camera: CameraDto {
            position: [0.0; 3],
            look_at: [0.0; 3],
            fov_deg: 0.0,
        },
        sky_bundle_path: None,
    };
    let json = serde_json::to_string(&scene).unwrap();
    let parsed: SceneDto = serde_json::from_str(&json).unwrap();
    assert_eq!(scene, parsed);
}

#[test]
fn test_material_stack_dto_roundtrip() {
    let stack = MaterialStackDto {
        id: 1,
        label: "concrete_wall".to_string(),
        layers: vec![MaterialLayerDto {
            archetype_id: 10,
            archetype_label: "Concrete".to_string(),
            thickness_mm: 200.0,
            coverage: 1.0,
        }],
    };
    let json = serde_json::to_string(&stack).unwrap();
    let parsed: MaterialStackDto = serde_json::from_str(&json).unwrap();
    assert_eq!(stack, parsed);
}

#[test]
fn test_impact_verdict_dto_roundtrip() {
    let verdicts = [
        ImpactVerdictDto::Stopped,
        ImpactVerdictDto::Penetrated,
        ImpactVerdictDto::Ricochet,
        ImpactVerdictDto::Embedded,
    ];
    for verdict in verdicts {
        let json = serde_json::to_string(&verdict).unwrap();
        let parsed: ImpactVerdictDto = serde_json::from_str(&json).unwrap();
        assert_eq!(verdict, parsed);
    }
}

#[test]
fn test_vertical_slice_observation_roundtrip() {
    let observation = VerticalSliceObservation {
        request_id: 42,
        scene: None,
        material_stacks: vec![],
        damage_memory: vec![],
        ballistic_result: None,
        runtime_events: vec![],
        shot_log: vec![],
    };
    let json = serde_json::to_string(&observation).unwrap();
    let parsed: VerticalSliceObservation = serde_json::from_str(&json).unwrap();
    assert_eq!(observation, parsed);
}

#[test]
fn test_runtime_event_dto_roundtrip() {
    let events = [
        RuntimeEventDto::ShotFired {
            tick: 100,
            weapon: 1,
        },
        RuntimeEventDto::ProjectileImpact {
            tick: 200,
            target: 2,
            energy_j: 50.0,
        },
        RuntimeEventDto::SegmentCracked {
            tick: 300,
            entity: 3,
            layer: 0,
            segment: 5,
        },
        RuntimeEventDto::SegmentReleased {
            tick: 400,
            entity: 4,
            layer: 1,
            segment: 10,
        },
    ];
    for event in events {
        let json = serde_json::to_string(&event).unwrap();
        let parsed: RuntimeEventDto = serde_json::from_str(&json).unwrap();
        assert_eq!(event, parsed);
    }
}

#[test]
fn test_shot_log_entry_dto_roundtrip() {
    let entry = ShotLogEntryDto {
        tick: 1000,
        weapon_entity: 1,
        projectile_profile: 5,
        spawn_position: [0.0, 1.5, 0.0],
        spawn_velocity: [0.0, 0.0, 500.0],
    };
    let json = serde_json::to_string(&entry).unwrap();
    let parsed: ShotLogEntryDto = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, parsed);
}

#[test]
fn test_ballistic_result_dto_roundtrip() {
    let result = BallisticSimulationResultDto { impacts: vec![] };
    let json = serde_json::to_string(&result).unwrap();
    let parsed: BallisticSimulationResultDto = serde_json::from_str(&json).unwrap();
    assert_eq!(result, parsed);
}

#[test]
fn test_damage_memory_dto_roundtrip() {
    let damage = DamageMemoryDto {
        entity_id: 1,
        stack_id: 1,
        layer_damage: vec![LayerDamageDto {
            layer_index: 0,
            integrity: 0.5,
            accumulated_energy_j: 100.0,
            cracked_segments: vec![1, 2, 3],
            released_segments: vec![],
        }],
    };
    let json = serde_json::to_string(&damage).unwrap();
    let parsed: DamageMemoryDto = serde_json::from_str(&json).unwrap();
    assert_eq!(damage, parsed);
}

#[test]
fn test_layer_impact_event_dto_roundtrip() {
    let event = LayerImpactEventDto {
        layer_index: 0,
        entry_energy_j: 500.0,
        exit_energy_j: 200.0,
        energy_absorbed_j: 300.0,
        verdict: ImpactVerdictDto::Penetrated,
    };
    let json = serde_json::to_string(&event).unwrap();
    let parsed: LayerImpactEventDto = serde_json::from_str(&json).unwrap();
    assert_eq!(event, parsed);
}

#[test]
fn test_impact_result_dto_roundtrip() {
    let result = ImpactResultDto {
        target_entity: 1,
        impact_position: [10.0, 5.0, 0.0],
        impact_velocity: [0.0, -10.0, 500.0],
        entry_energy_j: 1000.0,
        incidence_angle_deg: 45.0,
        layer_events: vec![],
        final_verdict: ImpactVerdictDto::Stopped,
    };
    let json = serde_json::to_string(&result).unwrap();
    let parsed: ImpactResultDto = serde_json::from_str(&json).unwrap();
    assert_eq!(result, parsed);
}
