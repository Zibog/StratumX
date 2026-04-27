mod command_ids;
mod manifest;
mod payloads;
mod route_domain;
mod route_metadata;
mod source_surface;
mod transaction_meta;

pub use command_ids::CommandId;
pub use manifest::{
    canonical_button_routes, canonical_route, canonical_route_by_action_id,
    canonical_route_manifest, CanonicalButtonRoute, CanonicalRouteManifest,
};
pub use payloads::{
    AudioPayload, CommandPayload, DiagnosticsPayload, EnvironmentPayload, ImportPayload,
    MaterialPayload, ProjectPayload, ShellPayload, TerrainPayload, WorldPayload,
};
pub use route_domain::RouteDomain;
pub use route_metadata::RouteMetadata;
pub use source_surface::SourceSurface;
pub use transaction_meta::TransactionMeta;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalCommandEnvelope<P = CommandPayload> {
    pub command_id: CommandId,
    pub payload: P,
    pub transaction_meta: TransactionMeta,
    pub source_surface: SourceSurface,
    pub route_metadata: RouteMetadata,
}

impl CanonicalCommandEnvelope<CommandPayload> {
    pub fn from_route(
        route: &CanonicalButtonRoute,
        payload: CommandPayload,
        source_surface: SourceSurface,
    ) -> Self {
        Self {
            command_id: CommandId::new(route.command_id.clone()),
            payload,
            transaction_meta: TransactionMeta::new(route.button_id.clone()),
            source_surface,
            route_metadata: RouteMetadata::from(route),
        }
    }

    pub fn domain(&self) -> RouteDomain {
        self.route_metadata.domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_deserializes_and_counts_match() {
        let manifest = canonical_route_manifest();
        assert_eq!(manifest.route_count, manifest.routes.len());
        assert_eq!(manifest.route_count, 85);
    }

    #[test]
    fn route_lookup_by_action_id_works() {
        let route = canonical_route_by_action_id("material.bind_light_response.requested")
            .expect("canonical action route");
        assert_eq!(route.button_id, "btn.material.bind_light_response");
    }
}
