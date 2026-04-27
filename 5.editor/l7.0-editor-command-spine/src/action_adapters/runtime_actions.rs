//! Runtime actions — tick control, pause, resume

use crate::command_spine::CommandSpine;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

/// Runtime action adapters for command spine
pub struct RuntimeActionAdapter<'a> {
    spine: &'a mut CommandSpine,
}

impl<'a> RuntimeActionAdapter<'a> {
    pub fn new(spine: &'a mut CommandSpine) -> Self {
        Self { spine }
    }

    pub fn request_pause(&mut self) {
        self.spine
            .submit_promoted_command(PromotedCommand::RuntimePause);
    }

    pub fn request_resume(&mut self) {
        self.spine
            .submit_promoted_command(PromotedCommand::RuntimeResume);
    }

    pub fn request_step_frame(&mut self) {
        self.spine
            .submit_promoted_command(PromotedCommand::RuntimeStepFrame);
    }
}
