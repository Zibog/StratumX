use serde::{Deserialize, Serialize};

use super::manifest::CanonicalButtonRoute;
use super::route_domain::RouteDomain;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteMetadata {
    pub button_id: String,
    pub action_id: String,
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
    pub status: String,
    pub domain: RouteDomain,
}

impl From<&CanonicalButtonRoute> for RouteMetadata {
    fn from(route: &CanonicalButtonRoute) -> Self {
        Self {
            button_id: route.button_id.clone(),
            action_id: route.action_id.clone(),
            executor_module: route.executor_module.clone(),
            owner_service: route.owner_service.clone(),
            owner_state: route.owner_state.clone(),
            publication_kinds: route.publication_kinds.clone(),
            persistence_kind: route.persistence_kind.clone(),
            diagnostics_kind: route.diagnostics_kind.clone(),
            focus_success: route.focus_success.clone(),
            focus_retry: route.focus_retry.clone(),
            focus_failure: route.focus_failure.clone(),
            recovery_anchor: route.recovery_anchor.clone(),
            evidence_family: route.evidence_family.clone(),
            status: route.status.clone(),
            domain: route.domain(),
        }
    }
}
