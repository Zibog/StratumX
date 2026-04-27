use std::collections::BTreeSet;

pub use stratumx_editor_l7_0_editor_command_spine::{
    ActionDenial, ActionRequest, CanonicalActionDispatch, CanonicalActionId, UiActionContext,
};
pub use stratumx_tooling_l6_0_tool_session::{
    CanonicalCommandExecutor, CommandResult, ToolSessionContext,
};
pub use stratumx_tooling_l6_1_command_envelopes::{
    canonical_button_routes, canonical_route, CanonicalButtonRoute, CanonicalCommandEnvelope,
    CommandPayload, SourceSurface,
};

pub fn manifest_button_ids() -> BTreeSet<&'static str> {
    canonical_button_routes()
        .iter()
        .map(|route| route.button_id.as_str())
        .collect()
}

pub fn route(button_id: &str) -> &'static CanonicalButtonRoute {
    canonical_route(button_id).expect("button route must exist")
}

pub fn routes_by_prefix(prefix: &str) -> Vec<&'static CanonicalButtonRoute> {
    canonical_button_routes()
        .iter()
        .filter(|route| route.button_id.starts_with(prefix))
        .collect()
}

pub fn dispatch_button(
    button_id: &str,
    payload: impl Into<CommandPayload>,
) -> Result<CanonicalCommandEnvelope<CommandPayload>, ActionDenial> {
    let route = route(button_id);
    let action_id = CanonicalActionId::from_action_id(&route.action_id)
        .expect("button route must have a cataloged action id");
    CanonicalActionDispatch::dispatch(ActionRequest {
        action_id,
        payload,
        ui_context: UiActionContext {
            source_surface: infer_source_surface(route),
            has_project: true,
            has_world: true,
            has_selection: true,
            shell_ready: true,
        },
    })
}

pub fn execute_button(
    button_id: &str,
    payload: impl Into<CommandPayload>,
) -> (ToolSessionContext, CommandResult) {
    let envelope = dispatch_button(button_id, payload).expect("button dispatch must succeed");
    let mut ctx = ToolSessionContext::default();
    let result = CanonicalCommandExecutor::execute(envelope, &mut ctx);
    (ctx, result)
}

pub fn execute_button_in_context(
    ctx: &mut ToolSessionContext,
    button_id: &str,
    payload: impl Into<CommandPayload>,
) -> CommandResult {
    let envelope = dispatch_button(button_id, payload).expect("button dispatch must succeed");
    CanonicalCommandExecutor::execute(envelope, ctx)
}

pub fn infer_source_surface(route: &CanonicalButtonRoute) -> SourceSurface {
    match route.button_id.split('.').nth(1).unwrap_or_default() {
        "project" | "world" => SourceSurface::MainMenu,
        "import" => SourceSurface::OpenWorldDialog,
        "terrain" => SourceSurface::TerrainPanel,
        "material" => SourceSurface::MaterialPanel,
        "sky" => SourceSurface::SkyPanel,
        "audio" => SourceSurface::AudioPanel,
        "view" => SourceSurface::ViewportPanel,
        _ => SourceSurface::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_support_executes_cataloged_button() {
        let (ctx, result) = execute_button("btn.material.bind_light_response", ());
        assert!(ctx
            .executed_buttons
            .iter()
            .any(|button| button == "btn.material.bind_light_response"));
        assert!(result.focus_target.is_some());
    }
}
