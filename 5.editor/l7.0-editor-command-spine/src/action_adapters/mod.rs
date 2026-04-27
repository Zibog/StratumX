//! Action Adapters — UI gesture → ActionId + payload → dispatch_action
//!
//! Per canon 74/75/82: UI must not call the runtime host facade directly.
//! All actions flow: UI → adapter → dispatch_action → command spine → executor → owner truth.

pub mod environment_actions;
pub mod runtime_actions;
pub mod world_actions;
