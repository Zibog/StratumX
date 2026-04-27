use crate::common::context::ToolSessionContext;
use crate::common::result::CommandResult;
use stratumx_tooling_l6_1_command_envelopes::{CanonicalCommandEnvelope, CommandPayload};

pub fn execute(
    envelope: &CanonicalCommandEnvelope<CommandPayload>,
    ctx: &mut ToolSessionContext,
) -> CommandResult {
    CommandResult::from_route_metadata(&envelope.route_metadata, ctx)
}
