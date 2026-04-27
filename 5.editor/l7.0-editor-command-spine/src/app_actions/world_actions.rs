// World lifecycle actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn open_world(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|world_path| PromotedCommand::WorldOpen { world_path })
}

pub fn save_current_world(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|world_path| PromotedCommand::WorldSave { world_path })
}

pub fn close_world(_payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    Some(PromotedCommand::WorldClose)
}
