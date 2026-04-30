use crate::types::{ConflictRule, PropertyType, WorldPropertySubstrate};

use super::SubstrateQuery;

impl SubstrateQuery {
    /// Get update order for properties.
    pub fn query_update_order(
        substrate: &WorldPropertySubstrate,
    ) -> Result<Vec<PropertyType>, String> {
        substrate.update_order.topological_sort()
    }

    /// Get all properties that depend on a given property.
    pub fn query_dependents(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> Vec<PropertyType> {
        substrate
            .update_order
            .edges
            .iter()
            .filter(|(from, _)| *from == property)
            .map(|(_, to)| *to)
            .collect()
    }

    /// Get all properties that a given property depends on.
    pub fn query_dependencies(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> Vec<PropertyType> {
        substrate
            .update_order
            .edges
            .iter()
            .filter(|(_, to)| *to == property)
            .map(|(from, _)| *from)
            .collect()
    }

    /// Get conflict rules involving a property.
    pub fn query_conflict_rules(
        substrate: &WorldPropertySubstrate,
        property: PropertyType,
    ) -> Vec<ConflictRule> {
        substrate
            .conflict_rules
            .iter()
            .filter(|rule| rule.properties.contains(&property))
            .cloned()
            .collect()
    }
}
