// Runtime control actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn play(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::RuntimePlay)
}

pub fn pause(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::RuntimePause)
}

pub fn stop(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::RuntimeStop)
}

pub fn simulate(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::RuntimeSimulate)
}
