use std::collections::HashMap;

use engine_material::{
    validate_conflict_rule, validate_coordinates, validate_dimensions, validate_field_value,
    validate_update_order, ConflictResolution, ConflictRule, PropertyType, UpdateOrderGraph,
};

#[test]
fn validate_dimensions_rejects_zero_and_oversized_axes() {
    assert!(validate_dimensions((10, 10, 10)).is_ok());
    assert!(validate_dimensions((0, 10, 10)).is_err());
    assert!(validate_dimensions((10, 0, 10)).is_err());
    assert!(validate_dimensions((10, 10, 0)).is_err());
    assert!(validate_dimensions((2000, 10, 10)).is_err());
}

#[test]
fn validate_coordinates_checks_bounds() {
    let dims = (10, 10, 10);
    assert!(validate_coordinates(5, 5, 5, dims).is_ok());
    assert!(validate_coordinates(0, 0, 0, dims).is_ok());
    assert!(validate_coordinates(9, 9, 9, dims).is_ok());
    assert!(validate_coordinates(10, 5, 5, dims).is_err());
    assert!(validate_coordinates(5, 10, 5, dims).is_err());
    assert!(validate_coordinates(5, 5, 10, dims).is_err());
}

#[test]
fn validate_field_value_enforces_property_ranges() {
    assert!(validate_field_value(PropertyType::Wetness, 0.5).is_ok());
    assert!(validate_field_value(PropertyType::Wetness, 0.0).is_ok());
    assert!(validate_field_value(PropertyType::Wetness, 1.0).is_ok());
    assert!(validate_field_value(PropertyType::Wetness, -0.1).is_err());
    assert!(validate_field_value(PropertyType::Wetness, 1.1).is_err());

    assert!(validate_field_value(PropertyType::Heat, 300.0).is_ok());
    assert!(validate_field_value(PropertyType::Heat, 0.0).is_ok());
    assert!(validate_field_value(PropertyType::Heat, -1.0).is_err());

    assert!(validate_field_value(PropertyType::Wetness, f32::NAN).is_err());
    assert!(validate_field_value(PropertyType::Wetness, f32::INFINITY).is_err());
}

#[test]
fn validate_update_order_detects_cycles() {
    let mut graph = UpdateOrderGraph::new();
    graph.add_edge(PropertyType::Heat, PropertyType::Wetness);
    assert!(validate_update_order(&graph).is_ok());

    graph.add_edge(PropertyType::Wetness, PropertyType::Heat);
    assert!(validate_update_order(&graph).is_err());
}

#[test]
fn validate_conflict_rule_requires_blend_weights() {
    let rule = ConflictRule {
        properties: vec![PropertyType::Heat, PropertyType::Wetness],
        resolution: ConflictResolution::Priority,
        weights: HashMap::new(),
    };
    assert!(validate_conflict_rule(&rule).is_ok());

    let rule = ConflictRule {
        properties: vec![PropertyType::Heat, PropertyType::Wetness],
        resolution: ConflictResolution::Blend,
        weights: HashMap::new(),
    };
    assert!(validate_conflict_rule(&rule).is_err());

    let mut weights = HashMap::new();
    weights.insert(PropertyType::Heat, 0.6);
    weights.insert(PropertyType::Wetness, 0.4);
    let rule = ConflictRule {
        properties: vec![PropertyType::Heat, PropertyType::Wetness],
        resolution: ConflictResolution::Blend,
        weights,
    };
    assert!(validate_conflict_rule(&rule).is_ok());
}
