use std::collections::HashMap;

use engine_material::{ConflictResolution, ConflictRule, PropertyType, SubstrateRuntime};

#[test]
fn substrate_runtime_updates_and_queries_cells() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::Wetness, (10, 10, 10), 0.0);

    runtime
        .update_cell_field(PropertyType::Wetness, 5, 5, 5, 0.75)
        .unwrap();

    let value = runtime
        .query_cell_field(PropertyType::Wetness, 5, 5, 5)
        .unwrap();
    assert_eq!(value, 0.75);
    assert!(runtime.dirty_properties.contains(&PropertyType::Wetness));
}

#[test]
fn substrate_runtime_round_trips_persistence() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
    runtime
        .update_cell_field(PropertyType::Heat, 2, 2, 2, 350.0)
        .unwrap();

    let data = runtime.save().unwrap();

    let mut restored = SubstrateRuntime::new();
    restored.load(&data).unwrap();

    let value = restored
        .query_cell_field(PropertyType::Heat, 2, 2, 2)
        .unwrap();
    assert_eq!(value, 350.0);
}

#[test]
fn substrate_runtime_partial_resume_merges_dirty_fields() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::Wetness, (5, 5, 5), 0.0);
    runtime.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
    runtime
        .update_cell_field(PropertyType::Wetness, 1, 1, 1, 0.5)
        .unwrap();

    let partial_data = runtime.save_partial().unwrap();

    let mut restored = SubstrateRuntime::new();
    restored.init_cell_field(PropertyType::Wetness, (5, 5, 5), 0.0);
    restored.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
    restored.load_partial(&partial_data).unwrap();

    let wetness = restored
        .query_cell_field(PropertyType::Wetness, 1, 1, 1)
        .unwrap();
    assert_eq!(wetness, 0.5);
}

#[test]
fn substrate_runtime_executes_update_order() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::Heat, (5, 5, 5), 300.0);
    runtime.init_cell_field(PropertyType::Wetness, (5, 5, 5), 0.5);

    assert!(runtime.update_all().is_ok());
}

#[test]
fn substrate_runtime_executes_topological_dependencies_deterministically() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::WindHint, (1, 1, 1), 1.0);
    runtime.init_cell_field(PropertyType::SmokeDensity, (1, 1, 1), 0.9);
    runtime.init_cell_field(PropertyType::VisibilityObscuration, (1, 1, 1), 0.0);

    runtime.update_all().unwrap();

    let smoke = runtime
        .query_cell_field(PropertyType::SmokeDensity, 0, 0, 0)
        .unwrap();
    let visibility = runtime
        .query_cell_field(PropertyType::VisibilityObscuration, 0, 0, 0)
        .unwrap();

    assert!((smoke - 0.72).abs() < 1e-6);
    assert!((visibility - 0.72).abs() < 1e-6);
}

#[test]
fn substrate_runtime_resolves_conflicts_into_canonical_target_field() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::Wetness, (1, 1, 1), 0.2);
    runtime.init_cell_field(PropertyType::SmokeDensity, (1, 1, 1), 0.8);
    runtime.substrate.conflict_rules.push(ConflictRule {
        properties: vec![PropertyType::Wetness, PropertyType::SmokeDensity],
        resolution: ConflictResolution::Blend,
        weights: HashMap::from([
            (PropertyType::Wetness, 0.25),
            (PropertyType::SmokeDensity, 0.75),
        ]),
    });

    runtime.resolve_conflicts().unwrap();

    let wetness = runtime
        .query_cell_field(PropertyType::Wetness, 0, 0, 0)
        .unwrap();
    let smoke = runtime
        .query_cell_field(PropertyType::SmokeDensity, 0, 0, 0)
        .unwrap();

    assert!((wetness - 0.65).abs() < 1e-6);
    assert!((smoke - 0.8).abs() < 1e-6);
}

#[test]
fn substrate_runtime_rejects_invalid_values_and_conflict_dimension_mismatch() {
    let mut runtime = SubstrateRuntime::new();
    runtime.init_cell_field(PropertyType::Wetness, (1, 1, 1), 0.2);
    runtime.init_cell_field(PropertyType::SmokeDensity, (2, 1, 1), 0.8);

    assert!(runtime
        .update_cell_field(PropertyType::Wetness, 0, 0, 0, 1.5)
        .is_err());

    runtime.substrate.conflict_rules.push(ConflictRule {
        properties: vec![PropertyType::Wetness, PropertyType::SmokeDensity],
        resolution: ConflictResolution::Maximum,
        weights: HashMap::new(),
    });

    assert!(runtime.resolve_conflicts().is_err());
}
