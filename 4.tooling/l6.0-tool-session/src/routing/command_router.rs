//! Command router — maps domain + command to executor result.
//!
//! Pure routing: no execution logic, no lifecycle, no serialization.

use super::command_domain::CommandDomain;
use crate::common::context::ToolSessionContext;
use crate::common::result::CommandResult;
use stratumx_tooling_l6_1_command_envelopes::{CanonicalCommandEnvelope, CommandPayload};

pub struct CommandRouter;

impl CommandRouter {
    pub fn route(
        domain: CommandDomain,
        envelope: &CanonicalCommandEnvelope<CommandPayload>,
        ctx: &mut ToolSessionContext,
    ) -> CommandResult {
        match domain {
            CommandDomain::Project | CommandDomain::World => {
                crate::world::executor_world::execute(envelope, ctx)
            }
            CommandDomain::Runtime => crate::runtime::executor_runtime::execute(envelope, ctx),
            CommandDomain::Terrain => crate::terrain::executor_terrain::execute(envelope, ctx),
            CommandDomain::Environment => {
                crate::environment::executor_environment::execute(envelope, ctx)
            }
            CommandDomain::Material => crate::material::executor_material::execute(envelope, ctx),
            CommandDomain::Audio => crate::audio::executor_audio::execute(envelope, ctx),
            CommandDomain::Build => crate::build::executor_build::execute(envelope, ctx),
            CommandDomain::Shell => crate::common::executor_shell::execute(envelope, ctx),
            CommandDomain::Scene => {
                CommandResult::from_route_metadata(&envelope.route_metadata, ctx)
            }
        }
    }
}
