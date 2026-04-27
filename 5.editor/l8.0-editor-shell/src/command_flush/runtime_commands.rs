//! Runtime State Command Handlers

use super::super::EditorApp;
use stratumx_editor_l8_0_editor_shell::status_bar::RuntimeState;

impl EditorApp {
    pub(super) fn handle_runtime_play(&mut self) -> Result<String, String> {
        self.state
            .shell
            .status_bar
            .set_runtime_state(RuntimeState::Playing);
        Ok("Runtime playing".into())
    }

    pub(super) fn handle_runtime_pause(&mut self) -> Result<String, String> {
        self.state
            .shell
            .status_bar
            .set_runtime_state(RuntimeState::Paused);
        Ok("Runtime paused".into())
    }

    pub(super) fn handle_runtime_stop(&mut self) -> Result<String, String> {
        self.state
            .shell
            .status_bar
            .set_runtime_state(RuntimeState::Editing);
        Ok("Runtime stopped".into())
    }

    pub(super) fn handle_runtime_simulate(&mut self) -> Result<String, String> {
        self.state
            .shell
            .status_bar
            .set_runtime_state(RuntimeState::Simulating);
        Ok("Runtime simulating".into())
    }
}
