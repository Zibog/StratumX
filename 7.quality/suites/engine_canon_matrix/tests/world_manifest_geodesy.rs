use engine_world::{ApplySegment, WorldId, WorldManifest, WorldRole, WorldState};
use engine_world_spatial::PrecisionZoneCode;

#[test]
fn world_manifest_validation_rejects_empty_namespace_and_zero_identity() {
    let mut manifest = WorldManifest::canonical_test_manifest();
    manifest.world_id = WorldId(0);
    manifest.namespace_roots.terrain.clear();

    assert!(WorldState::new_with_manifest(manifest).is_err());
}

#[test]
fn world_snapshot_carries_manifest_identity() {
    let mut manifest = WorldManifest::canonical_test_manifest();
    manifest.world_id = WorldId(77);
    manifest.world_role = WorldRole::PrimaryRuntime;

    let world = WorldState::new_with_manifest(manifest).unwrap();
    let snapshot = world.snapshot(0);

    assert_eq!(snapshot.world_id, WorldId(77));
}

#[test]
fn world_apply_publishes_rebase_and_far_causality() {
    let mut world = WorldState::new();

    world
        .apply(
            &[ApplySegment {
                region_key: (0, 0, 0),
                family_tags: vec![3],
            }],
            1,
        )
        .unwrap();
    world
        .apply(
            &[
                ApplySegment {
                    region_key: (6, 0, 0),
                    family_tags: vec![9],
                },
                ApplySegment {
                    region_key: (12, 0, 0),
                    family_tags: vec![11],
                },
            ],
            1,
        )
        .unwrap();

    assert!(world.last_rebase_delta_ref().is_some());
    assert_eq!(
        world.causal_summary().precision_zone_code,
        PrecisionZoneCode::FarSummary
    );
    assert_eq!(world.causal_summary().far_phenomenon_track_refs.len(), 1);
    assert!(world.causal_summary().near_region_frame_ref.is_some());
}
