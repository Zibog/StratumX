// Build and validation actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn run_build(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::BuildRun)
}

pub fn run_release(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::BuildRelease)
}

pub fn run_full_validation(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::ValidationRunFull)
}

pub fn run_smoke_validation(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::ValidationRunSmoke)
}

pub fn rebuild_all(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::AutomationRebuildAll)
}

pub fn validate_all(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::AutomationValidateAll)
}
