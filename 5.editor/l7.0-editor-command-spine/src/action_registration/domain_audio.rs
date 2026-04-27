//! Audio Domain Action Registration
//!
//! Registers audio authoring actions.

use crate::{ActionDefinition, ActionFamily, ActionId, ActionRegistry, MutationClass};

pub fn register(registry: &mut ActionRegistry) {
    registry.register(ActionDefinition {
        action_id: ActionId::new("audio.author_source"),
        display_label: "Author Audio Source".to_string(),
        action_family: ActionFamily::Audio,
        tooling_route: "route.audio.author_source.v1".to_string(),
        sdk_packet_family: "packet.audio.*".to_string(),
        engine_truth_owner: "tooling/audio".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "AUD_SOURCE_INVALID".to_string(),
        ],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("audio.configure_zone"),
        display_label: "Configure Audio Zone".to_string(),
        action_family: ActionFamily::Audio,
        tooling_route: "route.audio.configure_zone.v1".to_string(),
        sdk_packet_family: "packet.audio.*".to_string(),
        engine_truth_owner: "tooling/audio".to_string(),
        mutation_class: MutationClass::Mutate,
        possible_denial_families: vec!["WLD_NO_ACTIVE".to_string(), "AUD_ZONE_INVALID".to_string()],
    });

    registry.register(ActionDefinition {
        action_id: ActionId::new("audio.preview"),
        display_label: "Preview Audio".to_string(),
        action_family: ActionFamily::Audio,
        tooling_route: "route.audio.preview.v1".to_string(),
        sdk_packet_family: "packet.audio.*".to_string(),
        engine_truth_owner: "tooling/audio".to_string(),
        mutation_class: MutationClass::Capture,
        possible_denial_families: vec![
            "WLD_NO_ACTIVE".to_string(),
            "AUD_PREVIEW_UNAVAILABLE".to_string(),
        ],
    });
}
