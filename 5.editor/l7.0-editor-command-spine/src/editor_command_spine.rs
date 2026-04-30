//! Editor Command Spine - Façade and Public API
//!
//! This module provides the public API for the Command Spine architecture.
//!
//! ## Module Structure (Phase 2: Anti-Monolithic Decomposition)
//!
//! Core modules:
//! - action_ids, action_registry_types, action_context, action_result, action_enums
//! - action_preconditions, focus_recovery, action_dispatch_core
//! - action_registry, action_dispatch, action_registration

// ============================================================================
// Core Type Modules
// ============================================================================

mod action_context;
mod action_enums;
mod action_ids;
mod action_registry_types;
mod action_result;
mod canonical_action_api;

pub use action_context::{
    ActionContext, FocusState, ProjectState, SelectionState, SessionState, TransactionState,
    WorkspaceState,
};
pub use action_enums::{DenialFamily, DisabledReason, Locale};
pub use action_ids::ActionId;
pub use action_registry_types::{ActionDefinition, ActionFamily, MutationClass};
pub use action_result::{ActionResult, ErrorCode, FocusTarget};
pub use canonical_action_api::{
    ActionDenial, ActionRequest, CanonicalActionDispatch, CanonicalActionId, UiActionContext,
};

// ============================================================================
// Decomposed Modules
// ============================================================================

pub mod action_preconditions;
pub mod focus_recovery;

mod action_dispatch_core;
pub use action_dispatch_core::ActionDispatcher;

pub mod action_registry;
pub use action_registry::ActionRegistry;

mod registry_builder;
mod registry_core;
pub use registry_builder::{RegistrationError, RegistryBuilder};
pub use registry_core::ActionRegistry as ActionRegistryCore;

mod action_dispatch;
pub mod action_execution;
pub mod action_focus;
pub mod action_queries;
pub mod action_validation;

mod focus_routing;
pub use focus_routing::FocusRouter;

mod dispatch_audio;
mod dispatch_core;
mod dispatch_editor;
mod dispatch_material;
mod dispatch_route_family;
mod dispatch_world;

pub use action_dispatch::{
    ActionDispatch, ActionExecutor, ActionFocus, ActionValidator, StateQueries,
};
pub use dispatch_audio::AudioDispatch;
pub use dispatch_core::DispatchCore;
pub use dispatch_editor::EditorDispatch;
pub use dispatch_material::MaterialDispatch;
pub use dispatch_world::WorldDispatch;

// Command Spine - Central command routing with validation, preconditions, and history
mod command_spine;
mod command_types;
pub use command_spine::{
    ActionId as CommandActionId, Command, CommandId, CommandParameters, CommandResult,
    CommandSpine, CommandType, ExecutedCommand, ExecutionError, PreconditionError, RedoError,
    UndoData, UndoError, ValidationError,
};

mod action_registration;
pub use action_registration::register_core_actions;

// Command Execution - Environment, terrain, and project commands
pub mod command_execution;

// App Actions - Desktop UI command routing (feature-gated)
#[cfg(feature = "desktop")]
pub mod app_actions;

// ============================================================================
// Tests (inline unit tests only)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_action_id_creation() {
        let id = ActionId::new("world.open");
        assert_eq!(id.as_str(), "world.open");
    }

    #[test]
    fn test_action_context_predicates() {
        let context = ActionContext::test_with_project(Uuid::new_v4());
        assert!(context.has_project());
        assert!(!context.has_active_world());
    }

    #[test]
    fn test_action_result_constructors() {
        let success = ActionResult::success(FocusTarget::SpecificPanel("viewport".to_string()));
        assert!(success.success);

        let failure = ActionResult::failure(ErrorCode::UnknownAction, FocusTarget::Diagnostics);
        assert!(!failure.success);
    }
}
