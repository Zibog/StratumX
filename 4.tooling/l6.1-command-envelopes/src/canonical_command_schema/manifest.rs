use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

use super::route_domain::RouteDomain;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalRouteManifest {
    pub manifest_version: String,
    pub authoritative_doc: String,
    pub route_count: usize,
    pub routes: Vec<CanonicalButtonRoute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalButtonRoute {
    pub button_id: String,
    pub source_docs: Vec<String>,
    pub ui_location: String,
    pub action_id: String,
    pub command_id: String,
    pub envelope_type: String,
    pub executor_module: String,
    pub owner_service: String,
    pub owner_state: String,
    pub publication_kinds: Vec<String>,
    pub persistence_kind: String,
    pub diagnostics_kind: String,
    pub focus_success: String,
    pub focus_retry: String,
    pub focus_failure: String,
    pub recovery_anchor: String,
    pub evidence_family: String,
    pub test_ids: Vec<String>,
    pub status: String,
}

impl CanonicalButtonRoute {
    pub fn domain(&self) -> RouteDomain {
        match self.button_id.split('.').nth(1).unwrap_or_default() {
            "project" => RouteDomain::Project,
            "world" => RouteDomain::World,
            "import" => RouteDomain::Import,
            "terrain" => RouteDomain::Terrain,
            "material" => RouteDomain::Material,
            "sky" => RouteDomain::Environment,
            "view" => RouteDomain::Shell,
            "audio" => RouteDomain::Audio,
            _ => RouteDomain::Diagnostics,
        }
    }
}

pub fn canonical_route_manifest() -> &'static CanonicalRouteManifest {
    static MANIFEST: OnceLock<CanonicalRouteManifest> = OnceLock::new();
    MANIFEST.get_or_init(|| {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../7.quality/suites/editor_command_matrix/button_route_coverage.json"
        )))
        .expect("phase4 route manifest must deserialize")
    })
}

pub fn canonical_button_routes() -> &'static [CanonicalButtonRoute] {
    &canonical_route_manifest().routes
}

pub fn canonical_route(button_id: &str) -> Option<&'static CanonicalButtonRoute> {
    canonical_button_routes()
        .iter()
        .find(|route| route.button_id == button_id)
}

pub fn canonical_route_by_action_id(action_id: &str) -> Option<&'static CanonicalButtonRoute> {
    canonical_button_routes()
        .iter()
        .find(|route| route.action_id == action_id)
}
