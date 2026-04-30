use engine_material::{PropertyType, UpdateOrderGraph, WorldPropertySubstrate};

#[test]
fn update_order_graph_topological_sort_respects_dependencies() {
    let mut graph = UpdateOrderGraph::new();
    graph.add_edge(PropertyType::Heat, PropertyType::Wetness);
    graph.add_edge(PropertyType::WindHint, PropertyType::SmokeDensity);
    graph.add_edge(
        PropertyType::SmokeDensity,
        PropertyType::VisibilityObscuration,
    );

    let order = graph.topological_sort().unwrap();

    let heat_idx = order.iter().position(|&p| p == PropertyType::Heat).unwrap();
    let wetness_idx = order
        .iter()
        .position(|&p| p == PropertyType::Wetness)
        .unwrap();
    assert!(heat_idx < wetness_idx);

    let wind_idx = order
        .iter()
        .position(|&p| p == PropertyType::WindHint)
        .unwrap();
    let smoke_idx = order
        .iter()
        .position(|&p| p == PropertyType::SmokeDensity)
        .unwrap();
    let visibility_idx = order
        .iter()
        .position(|&p| p == PropertyType::VisibilityObscuration)
        .unwrap();
    assert!(wind_idx < smoke_idx);
    assert!(smoke_idx < visibility_idx);
}

#[test]
fn update_order_graph_detects_cycles() {
    let mut graph = UpdateOrderGraph::new();
    graph.add_edge(PropertyType::Heat, PropertyType::Wetness);
    graph.add_edge(PropertyType::Wetness, PropertyType::Heat);

    let result = graph.topological_sort();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Cycle detected"));
}

#[test]
fn world_property_substrate_creation_starts_empty() {
    let substrate = WorldPropertySubstrate::new();
    assert!(substrate.cell_fields.is_empty());
    assert!(substrate.object_local_fields.is_empty());
    assert!(substrate.surface_fields.is_empty());
    assert!(substrate.volume_fields.is_empty());
}

#[test]
fn default_update_order_keeps_heat_before_wetness() {
    let substrate = WorldPropertySubstrate::with_default_update_order();
    let order = substrate.update_order.topological_sort().unwrap();

    let heat_idx = order.iter().position(|&p| p == PropertyType::Heat);
    let wetness_idx = order.iter().position(|&p| p == PropertyType::Wetness);

    if let (Some(heat_idx), Some(wetness_idx)) = (heat_idx, wetness_idx) {
        assert!(heat_idx < wetness_idx, "Heat should update before Wetness");
    }
}
