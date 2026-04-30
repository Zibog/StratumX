use super::domains::PromotedCommand;
use super::ids::*;

pub(crate) fn route_id(command: &PromotedCommand) -> Option<&'static str> {
    Some(match command {
        PromotedCommand::ShellActivateViewport => ROUTE_SHELL_ACTIVATE_VIEWPORT,
        PromotedCommand::ShellActivateOutliner => ROUTE_SHELL_ACTIVATE_OUTLINER,
        PromotedCommand::ShellActivateInspector => ROUTE_SHELL_ACTIVATE_INSPECTOR,
        PromotedCommand::ShellActivateContentBrowser => ROUTE_SHELL_ACTIVATE_CONTENT_BROWSER,
        PromotedCommand::ShellActivateMaterialLab => ROUTE_SHELL_ACTIVATE_MATERIAL_SURFACE,
        PromotedCommand::ShellActivateTerrainLab => ROUTE_SHELL_ACTIVATE_TERRAIN_SURFACE,
        PromotedCommand::ShellActivateSkyLab => ROUTE_SHELL_ACTIVATE_SKY_SURFACE,
        _ => return None,
    })
}
