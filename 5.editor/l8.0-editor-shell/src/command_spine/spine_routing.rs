//! Command Spine Routing Logic
//!
//! This module contains the routing logic for mapping commands to their appropriate
//! executors. The routing system ensures that each command is handled by the correct
//! domain-specific executor based on the command's route identifier.
//!
//! ## Responsibilities
//! - Route commands to appropriate domain executors
//! - Maintain executor registry for command dispatch
//! - Provide executor lookup based on command characteristics
//!
//! ## Executor Domains
//! - World lifecycle (open, close, save)
//! - Runtime control (play, pause, step)
//! - Terrain manipulation (sculpt, paint, configure)
//! - Environment (sky, weather, time)
//! - Material authoring (author, bind, inspect)
//! - Audio authoring (author source, configure zone, preview)
//! - Build/validation (validate, bake, package)
//! - Shell/project (project open/close, workspace save)

use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

/// Routing structure for mapping commands to executors
pub struct SpineRouting {
    // Executor registry is introduced once concrete domain executors are attached.
}

impl SpineRouting {
    /// Creates a new routing instance with default executor registry
    pub fn new() -> Self {
        Self {}
    }

    /// Routes a command to the appropriate executor based on route identifier
    ///
    /// # Arguments
    /// * `command` - The promoted command to route
    ///
    /// # Returns
    /// The executor identifier for the command's domain
    ///
    /// # Note
    /// Current routing classifies commands by route-id substring until concrete executor lookup is attached.
    pub fn route_command(&self, command: &PromotedCommand) -> ExecutorId {
        // Route based on command route identifier
        // This will be expanded when domain executors are implemented
        let route_id = command.route_id();

        if route_id.contains("world") {
            ExecutorId::World
        } else if route_id.contains("runtime") {
            ExecutorId::Runtime
        } else if route_id.contains("terrain") {
            ExecutorId::Terrain
        } else if route_id.contains("environment") {
            ExecutorId::Environment
        } else if route_id.contains("material") {
            ExecutorId::Material
        } else if route_id.contains("audio") {
            ExecutorId::Audio
        } else if route_id.contains("build") {
            ExecutorId::Build
        } else if route_id.contains("project") || route_id.contains("workspace") {
            ExecutorId::Shell
        } else {
            ExecutorId::Default
        }
    }

    /// Gets the executor for a specific command
    ///
    /// # Arguments
    /// * `command` - The promoted command
    ///
    /// # Returns
    /// The executor identifier for handling this command
    pub fn get_executor_for_command(&self, command: &PromotedCommand) -> ExecutorId {
        self.route_command(command)
    }
}

impl Default for SpineRouting {
    fn default() -> Self {
        Self::new()
    }
}

/// Identifier for domain-specific executors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutorId {
    /// World lifecycle executor (open, close, save)
    World,
    /// Runtime control executor (play, pause, step)
    Runtime,
    /// Terrain manipulation executor (sculpt, paint, configure)
    Terrain,
    /// Environment executor (sky, weather, time)
    Environment,
    /// Material authoring executor (author, bind, inspect)
    Material,
    /// Audio authoring executor (author source, configure zone, preview)
    Audio,
    /// Build/validation executor (validate, bake, package)
    Build,
    /// Shell/project executor (project open/close, workspace save)
    Shell,
    /// Default executor for unrecognized commands
    Default,
}
