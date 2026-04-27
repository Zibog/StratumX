//! Command Spine Core Logic
//!
//! This module contains the core CommandSpine struct and its primary execution logic.
//! The CommandSpine enforces canonical routing from UI actions through tooling routes
//! to SDK packets and engine truth owners.
//!
//! ## Responsibilities
//! - Execute promoted commands through the command executor
//! - Manage command lifecycle state tracking
//! - Coordinate diagnostics publishing for command execution
//! - Provide query interfaces for command state and diagnostics
//!
//! ## Canonical Path
//! UI → Action → Command → Spine → Tooling Executor → Authority → SDK → Engine

use crate::editor_product_model::EditorProduct;
use stratumx_tooling_l6_0_tool_session::CommandExecutor;
use stratumx_tooling_l6_0_tool_session::ToolingError;
use stratumx_tooling_l6_1_command_envelopes::{CommandLifecycleState, PromotedCommand};
use stratumx_tooling_l6_6_tool_diagnostics_events::{DiagnosticSeverity, DiagnosticsPublisher};

/// Core command spine structure managing command execution and diagnostics
pub struct CommandSpine {
    executor: CommandExecutor,
    diagnostics: DiagnosticsPublisher,
}

impl std::fmt::Debug for CommandSpine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandSpine")
            .field("executor", &"<CommandExecutor>")
            .field("diagnostics", &"<DiagnosticsPublisher>")
            .finish()
    }
}

impl Default for CommandSpine {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandSpine {
    /// Creates a new CommandSpine instance with default executor and diagnostics
    pub fn new() -> Self {
        Self {
            executor: CommandExecutor::new(),
            diagnostics: DiagnosticsPublisher::new(),
        }
    }

    /// Executes a promoted command through the canonical routing path
    ///
    /// # Arguments
    /// * `command` - The promoted command to execute
    /// * `product` - Mutable reference to the editor product for tooling access
    ///
    /// # Returns
    /// * `Ok(command_id)` - The unique identifier for the submitted command
    /// * `Err(ToolingError)` - If command submission or execution fails
    pub fn execute_promoted_command(
        &mut self,
        command: PromotedCommand,
        product: &mut EditorProduct,
    ) -> Result<u64, ToolingError> {
        let command_id = self.executor.submit_command(command.clone())?;

        self.diagnostics.publish_with_command(
            DiagnosticSeverity::Info,
            "command_spine",
            format!("Command {} submitted: {:?}", command_id, command.route_id()),
            command_id,
        );

        let result = self
            .executor
            .dispatch_command(command_id, &mut product.tooling);

        match result {
            Ok(_) => {
                self.diagnostics.publish_with_command(
                    DiagnosticSeverity::Info,
                    "command_spine",
                    format!("Command {} succeeded", command_id),
                    command_id,
                );
                product.refresh_from_tooling()?;
            }
            Err(ref e) => {
                self.diagnostics.publish_with_command(
                    DiagnosticSeverity::Error,
                    "command_spine",
                    format!("Command {} failed: {}", command_id, e),
                    command_id,
                );
            }
        }

        Ok(command_id)
    }

    /// Queries the current lifecycle state of a command
    ///
    /// # Arguments
    /// * `command_id` - The unique identifier of the command
    ///
    /// # Returns
    /// * `Some(CommandLifecycleState)` - If the command exists
    /// * `None` - If the command is not found
    pub fn get_command_state(&self, command_id: u64) -> Option<CommandLifecycleState> {
        self.executor
            .get_envelope(command_id)
            .map(|e| e.lifecycle_state)
    }

    /// Retrieves all diagnostics associated with a specific command
    ///
    /// # Arguments
    /// * `command_id` - The unique identifier of the command
    ///
    /// # Returns
    /// A vector of formatted diagnostic messages for the command
    pub fn get_diagnostics_for_command(&self, command_id: u64) -> Vec<String> {
        self.diagnostics
            .events_for_command(command_id)
            .iter()
            .map(|e| format!("[{:?}] {}: {}", e.severity, e.source, e.message))
            .collect()
    }

    /// Retrieves all diagnostics from the diagnostics publisher
    ///
    /// # Returns
    /// A vector of all formatted diagnostic messages
    pub fn all_diagnostics(&self) -> Vec<String> {
        self.diagnostics
            .events()
            .iter()
            .map(|e| format!("[{:?}] {}: {}", e.severity, e.source, e.message))
            .collect()
    }

    /// Provides read-only access to the command executor
    pub fn executor(&self) -> &CommandExecutor {
        &self.executor
    }

    /// Provides read-only access to the diagnostics publisher
    pub fn diagnostics(&self) -> &DiagnosticsPublisher {
        &self.diagnostics
    }
}
