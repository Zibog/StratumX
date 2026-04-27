//! World actions — save, load, snapshot

use crate::command_spine::CommandSpine;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

/// World action adapters for command spine
pub struct WorldActionAdapter<'a> {
    spine: &'a mut CommandSpine,
}

impl<'a> WorldActionAdapter<'a> {
    pub fn new(spine: &'a mut CommandSpine) -> Self {
        Self { spine }
    }

    pub fn request_save_world(&mut self, path: String) {
        self.spine
            .submit_promoted_command(PromotedCommand::WorldSave { path });
    }

    pub fn request_load_world(&mut self, path: String) {
        self.spine
            .submit_promoted_command(PromotedCommand::WorldLoad { path });
    }

    pub fn request_snapshot(&mut self) {
        self.spine
            .submit_promoted_command(PromotedCommand::WorldSnapshot);
    }
}
