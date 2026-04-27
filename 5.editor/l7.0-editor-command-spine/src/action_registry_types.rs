//! Action Registry Types - Core types for action registration
//!
//! This module defines the types used for registering and managing actions
//! in the command spine registry.

use crate::action_ids::ActionId;
use std::fmt;

// ============================================================================
// Action Classification
// ============================================================================

/// Categorizes actions by their domain or functional area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionFamily {
    ProjectFile,
    ViewPanel,
    World,
    Terrain,
    Material,
    Audio,
    SkyEnvironment,
    Runtime,
    DiagnosticsProof,
    BuildRelease,
}

/// Classifies the mutation impact of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationClass {
    Read,
    Mutate,
    Simulate,
    Compare,
    Capture,
    Recover,
    Certify,
}

// ============================================================================
// Action Definition
// ============================================================================

/// Complete definition of an action including routing and metadata.
///
/// Per canonical boundary law (09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX):
/// - Editor defines routes and metadata
/// - Tooling owns transaction execution
/// - SDK owns typed transport
/// - Engine owns runtime truth
#[derive(Debug, Clone)]
pub struct ActionDefinition {
    pub action_id: ActionId,
    pub display_label: String,
    pub action_family: ActionFamily,
    pub tooling_route: String,
    pub sdk_packet_family: String,
    pub engine_truth_owner: String,
    pub mutation_class: MutationClass,
    pub possible_denial_families: Vec<String>,
}

impl fmt::Display for ActionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActionDefinition {{ id: {}, label: {}, family: {:?}, route: {} }}",
            self.action_id, self.display_label, self.action_family, self.tooling_route
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_family_variants() {
        let families = vec![
            ActionFamily::ProjectFile,
            ActionFamily::ViewPanel,
            ActionFamily::World,
            ActionFamily::Terrain,
            ActionFamily::Material,
            ActionFamily::Audio,
            ActionFamily::SkyEnvironment,
            ActionFamily::Runtime,
            ActionFamily::DiagnosticsProof,
            ActionFamily::BuildRelease,
        ];
        assert_eq!(families.len(), 10);
    }

    #[test]
    fn test_mutation_class_variants() {
        let classes = [
            MutationClass::Read,
            MutationClass::Mutate,
            MutationClass::Simulate,
            MutationClass::Compare,
            MutationClass::Capture,
            MutationClass::Recover,
            MutationClass::Certify,
        ];
        assert_eq!(classes.len(), 7);
    }

    #[test]
    fn test_action_definition_display() {
        let def = ActionDefinition {
            action_id: ActionId::new("world.open"),
            display_label: "Open World".to_string(),
            action_family: ActionFamily::World,
            tooling_route: "route.world.open.v1".to_string(),
            sdk_packet_family: "packet.world.*".to_string(),
            engine_truth_owner: "world_truth".to_string(),
            mutation_class: MutationClass::Mutate,
            possible_denial_families: vec![],
        };

        let display = format!("{}", def);
        assert!(display.contains("world.open"));
        assert!(display.contains("Open World"));
    }
}
