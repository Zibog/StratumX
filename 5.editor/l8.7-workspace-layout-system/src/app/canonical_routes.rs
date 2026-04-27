//! Canonical Routes — single entry point for promoted commands.
//!
//! Per canon 74/75/82: this is the ONLY lower bound for command submission.
//! It delegates to `self.state.shell.submit_command(...)` which routes through
//! the tooling spine. No recursion, no special-case routing.
//!
//! UI → action_adapters → submit_promoted_command → shell.submit_command → tooling spine

use super::EditorApp;
use stratumx_editor_l8_0_editor_shell::status_bar::MessageType;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

impl EditorApp {
    /// UI payload lowering: action_id + payload → PromotedCommand → submit.
    pub fn dispatch_phase4_action(
        &mut self,
        action_id: &str,
        payload: Option<crate::editor_actions::ActionPayload>,
    ) {
        if let Some(command) = crate::editor_actions::ActionDispatcher::dispatch(action_id, payload)
        {
            self.submit_promoted_command(command);
        }
    }

    /// Submit promoted command — the single lower bound for all command routing.
    /// NEVER calls back into adapter layer. Always delegates to shell.submit_command.
    pub fn submit_promoted_command(&mut self, command: PromotedCommand) {
        match self.state.shell.submit_command(command) {
            Ok(command_id) => {
                self.state.ui_status_message =
                    Some(format!("Canonical command submitted: {}", command_id));
            }
            Err(error) => {
                self.state.ui_status_message = Some(format!("Canonical command failed: {}", error));
                self.state
                    .shell
                    .set_status(format!("Command failed: {}", error), MessageType::Error);
            }
        }
    }
}
