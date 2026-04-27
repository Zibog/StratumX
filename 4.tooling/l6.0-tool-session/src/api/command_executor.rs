use crate::build::executor_build;
use crate::common::context::ToolSessionContext;
use crate::common::executor_shell;
use crate::common::result::CommandResult;
use crate::diagnostics::executor_diagnostics;
use crate::environment::executor_environment;
use crate::material::executor_material;
use crate::runtime::executor_runtime;
use crate::terrain::executor_terrain;
use crate::world::executor_world;
use stratumx_tooling_l6_1_command_envelopes::{
    CanonicalCommandEnvelope, CommandPayload, RouteDomain,
};

pub struct CanonicalCommandExecutor;

impl CanonicalCommandExecutor {
    pub fn execute(
        envelope: CanonicalCommandEnvelope<CommandPayload>,
        ctx: &mut ToolSessionContext,
    ) -> CommandResult {
        match envelope.domain() {
            RouteDomain::Project | RouteDomain::World | RouteDomain::Import => {
                executor_world::execute(&envelope, ctx)
            }
            RouteDomain::Terrain => executor_terrain::execute(&envelope, ctx),
            RouteDomain::Material => executor_material::execute(&envelope, ctx),
            RouteDomain::Environment => executor_environment::execute(&envelope, ctx),
            RouteDomain::Shell => executor_shell::execute(&envelope, ctx),
            RouteDomain::Audio => executor_audio::execute(&envelope, ctx),
            RouteDomain::Runtime => executor_runtime::execute(&envelope, ctx),
            RouteDomain::Build => executor_build::execute(&envelope, ctx),
            RouteDomain::Diagnostics => executor_diagnostics::execute(&envelope, ctx),
        }
    }
}

mod executor_audio {
    use crate::common::context::ToolSessionContext;
    use crate::common::result::CommandResult;
    use stratumx_tooling_l6_1_command_envelopes::{CanonicalCommandEnvelope, CommandPayload};

    pub fn execute(
        envelope: &CanonicalCommandEnvelope<CommandPayload>,
        ctx: &mut ToolSessionContext,
    ) -> CommandResult {
        CommandResult::from_route_metadata(&envelope.route_metadata, ctx)
    }
}
