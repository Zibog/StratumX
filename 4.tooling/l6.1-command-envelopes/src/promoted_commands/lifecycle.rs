use super::domains::PromotedCommand;
use super::ids::*;

pub(crate) fn route_id(command: &PromotedCommand) -> Option<&'static str> {
    Some(match command {
        PromotedCommand::ProjectBootstrap { .. } => ROUTE_PROJECT_BOOTSTRAP,
        PromotedCommand::ProjectCreate { .. } => ROUTE_PROJECT_CREATE,
        PromotedCommand::ProjectSave { .. } => ROUTE_PROJECT_SAVE,
        PromotedCommand::ProjectBuild { .. } => ROUTE_BUILD_PACKAGE,
        PromotedCommand::ProjectExport { .. } => ROUTE_EXPORT_TARGET,
        PromotedCommand::ProjectLaunch { .. } | PromotedCommand::ProjectVerifyFirstResult => {
            ROUTE_LAUNCH_VERIFY_FIRST_RESULT
        }
        PromotedCommand::WorldOpen { .. } => ROUTE_WORLD_OPEN,
        PromotedCommand::WorldSave { .. } => ROUTE_WORLD_SAVE,
        PromotedCommand::WorldClose => ROUTE_WORLD_CLOSE,
        PromotedCommand::RuntimePlay => ROUTE_RUNTIME_PLAY,
        PromotedCommand::RuntimePause => ROUTE_RUNTIME_PAUSE,
        PromotedCommand::RuntimeStop => ROUTE_RUNTIME_STOP,
        PromotedCommand::RuntimeSimulate => ROUTE_RUNTIME_SIMULATE,
        PromotedCommand::MaterialAuthorityInitialize => ROUTE_MATERIAL_AUTHORITY_INITIALIZE,
        PromotedCommand::MaterialAuthorityDispose => ROUTE_MATERIAL_AUTHORITY_DISPOSE,
        PromotedCommand::AudioAuthorityInitialize => ROUTE_AUDIO_AUTHORITY_INITIALIZE,
        PromotedCommand::AudioAuthorityDispose => ROUTE_AUDIO_AUTHORITY_DISPOSE,
        PromotedCommand::BuildRun => ROUTE_BUILD_RUN,
        PromotedCommand::BuildRelease => ROUTE_BUILD_RELEASE,
        PromotedCommand::ValidationRunFull => ROUTE_WORLD_VALIDATE,
        PromotedCommand::ValidationRunSmoke => ROUTE_VALIDATION_SMOKE,
        PromotedCommand::AutomationRebuildAll => ROUTE_AUTOMATION_REBUILD_ALL,
        PromotedCommand::AutomationValidateAll => ROUTE_AUTOMATION_VALIDATE_ALL,
        PromotedCommand::SceneBootstrap => ROUTE_SCENE_BOOTSTRAP,
        PromotedCommand::SceneFireTestShot { .. } => ROUTE_SCENE_FIRE_TEST_SHOT,
        PromotedCommand::SceneReset => ROUTE_SCENE_RESET,
        _ => return None,
    })
}
