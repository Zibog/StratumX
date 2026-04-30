use engine_material::{
    CellField, ObjectLocalField, PropertyType, SubstrateQuery, WorldPropertySubstrate,
};

fn create_test_substrate() -> WorldPropertySubstrate {
    let mut substrate = WorldPropertySubstrate::new();

    substrate.cell_fields.insert(
        PropertyType::Wetness,
        CellField {
            property: PropertyType::Wetness,
            dimensions: (5, 5, 5),
            data: vec![0.5; 125],
        },
    );

    substrate.object_local_fields.insert(
        1,
        vec![ObjectLocalField {
            property: PropertyType::Heat,
            entity_id: 1,
            value: 300.0,
        }],
    );

    substrate
        .update_order
        .add_edge(PropertyType::Heat, PropertyType::Wetness);

    substrate
}

#[test]
fn query_cell_property_returns_expected_values() {
    let substrate = create_test_substrate();
    let value = SubstrateQuery::query_cell_property(&substrate, PropertyType::Wetness, 2, 2, 2);
    assert_eq!(value, Some(0.5));

    let value = SubstrateQuery::query_cell_property(&substrate, PropertyType::Heat, 2, 2, 2);
    assert_eq!(value, None);
}

#[test]
fn query_cell_properties_collects_present_cell_values() {
    let substrate = create_test_substrate();
    let properties = SubstrateQuery::query_cell_properties(&substrate, 2, 2, 2);
    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].0, PropertyType::Wetness);
    assert_eq!(properties[0].1, 0.5);
}

#[test]
fn query_object_property_reads_object_local_fields() {
    let substrate = create_test_substrate();
    let value = SubstrateQuery::query_object_property(&substrate, 1, PropertyType::Heat);
    assert_eq!(value, Some(300.0));

    let value = SubstrateQuery::query_object_property(&substrate, 2, PropertyType::Heat);
    assert_eq!(value, None);
}

#[test]
fn query_property_everywhere_collects_storage_views() {
    let substrate = create_test_substrate();
    let result = SubstrateQuery::query_property_everywhere(&substrate, PropertyType::Wetness);

    assert!(result.cell_field.is_some());
    assert_eq!(result.object_fields.len(), 0);
    assert_eq!(result.surface_fields.len(), 0);
    assert_eq!(result.volume_fields.len(), 0);
}

#[test]
fn query_update_order_returns_dependency_order() {
    let substrate = create_test_substrate();
    let order = SubstrateQuery::query_update_order(&substrate).unwrap();

    let heat_idx = order.iter().position(|&p| p == PropertyType::Heat).unwrap();
    let wetness_idx = order
        .iter()
        .position(|&p| p == PropertyType::Wetness)
        .unwrap();
    assert!(heat_idx < wetness_idx);
}

#[test]
fn query_dependents_returns_forward_edges() {
    let substrate = create_test_substrate();
    let dependents = SubstrateQuery::query_dependents(&substrate, PropertyType::Heat);
    assert_eq!(dependents, vec![PropertyType::Wetness]);
}

#[test]
fn query_dependencies_returns_reverse_edges() {
    let substrate = create_test_substrate();
    let dependencies = SubstrateQuery::query_dependencies(&substrate, PropertyType::Wetness);
    assert_eq!(dependencies, vec![PropertyType::Heat]);
}
