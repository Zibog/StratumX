//! Action Registration - Registers initial core actions with the ActionRegistry
//!
//! This module provides functions to register core editor actions with their
//! handlers, tooling route mappings, and metadata.
//!
//! The registration is split into domain-specific modules:
//! - domain_world: World lifecycle actions
//! - domain_terrain: Terrain manipulation actions
//! - domain_material: Material authoring actions
//! - domain_shell: Project and panel management actions
//! - domain_runtime: Runtime control actions
//! - domain_diagnostics: Diagnostics actions
//! - domain_build: Build and release actions

use crate::ActionRegistry;

mod domain_audio;
mod domain_build;
mod domain_diagnostics;
mod domain_material;
mod domain_runtime;
mod domain_shell;
mod domain_terrain;
mod domain_world;

/// Registers all initial core action handlers with the ActionRegistry.
///
/// This function delegates to domain-specific registration modules:
/// - domain_world: World lifecycle actions
/// - domain_terrain: Terrain manipulation actions
/// - domain_material: Material authoring actions
/// - domain_shell: Project and panel management actions
/// - domain_runtime: Runtime control actions
/// - domain_diagnostics: Diagnostics actions
/// - domain_build: Build and release actions
///
/// Each action is registered with:
/// - Action ID and display label
/// - Action family categorization
/// - Tooling route mapping
/// - SDK packet family
/// - Engine truth owner
/// - Mutation class
/// - Possible denial families
pub fn register_core_actions(registry: &mut ActionRegistry) {
    domain_world::register(registry);
    domain_terrain::register(registry);
    domain_material::register(registry);
    domain_audio::register(registry);
    domain_shell::register(registry);
    domain_runtime::register(registry);
    domain_diagnostics::register(registry);
    domain_build::register(registry);
}
