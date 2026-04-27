// Command builder helpers for constructing promoted commands

use super::types::PromotedCommand;

/// Builder for project commands
pub struct ProjectCommandBuilder;

impl ProjectCommandBuilder {
    pub fn bootstrap(project_name: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectBootstrap {
            project_name: project_name.into(),
        }
    }

    pub fn create(
        project_name: impl Into<String>,
        project_root: impl Into<String>,
        world_name: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::ProjectCreate {
            project_name: project_name.into(),
            project_root: project_root.into(),
            world_name: world_name.into(),
        }
    }

    pub fn save(save_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectSave {
            save_path: save_path.into(),
        }
    }

    pub fn build(target_platform: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectBuild {
            target_platform: target_platform.into(),
        }
    }

    pub fn export(export_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectExport {
            export_path: export_path.into(),
        }
    }

    pub fn launch(launch_mode: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectLaunch {
            launch_mode: launch_mode.into(),
        }
    }

    pub fn verify_first_result() -> PromotedCommand {
        PromotedCommand::ProjectVerifyFirstResult
    }
}

/// Builder for world commands
pub struct WorldCommandBuilder;

impl WorldCommandBuilder {
    pub fn open(world_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::WorldOpen {
            world_path: world_path.into(),
        }
    }

    pub fn save(world_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::WorldSave {
            world_path: world_path.into(),
        }
    }

    pub fn close() -> PromotedCommand {
        PromotedCommand::WorldClose
    }
}

/// Builder for runtime commands
pub struct RuntimeCommandBuilder;

impl RuntimeCommandBuilder {
    pub fn play() -> PromotedCommand {
        PromotedCommand::RuntimePlay
    }

    pub fn pause() -> PromotedCommand {
        PromotedCommand::RuntimePause
    }

    pub fn stop() -> PromotedCommand {
        PromotedCommand::RuntimeStop
    }

    pub fn simulate() -> PromotedCommand {
        PromotedCommand::RuntimeSimulate
    }
}

/// Builder for terrain commands
pub struct TerrainCommandBuilder;

impl TerrainCommandBuilder {
    pub fn import(heightmap_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::TerrainImport {
            heightmap_path: heightmap_path.into(),
        }
    }

    pub fn rebuild() -> PromotedCommand {
        PromotedCommand::TerrainRebuild
    }

    pub fn sculpt_raise(position: [f32; 2], radius: f32, strength: f32) -> PromotedCommand {
        PromotedCommand::TerrainSculptRaise {
            position,
            radius,
            strength,
        }
    }

    pub fn sculpt_lower(position: [f32; 2], radius: f32, strength: f32) -> PromotedCommand {
        PromotedCommand::TerrainSculptLower {
            position,
            radius,
            strength,
        }
    }

    pub fn sculpt_smooth(position: [f32; 2], radius: f32, strength: f32) -> PromotedCommand {
        PromotedCommand::TerrainSculptSmooth {
            position,
            radius,
            strength,
        }
    }

    pub fn sculpt_flatten(
        position: [f32; 2],
        radius: f32,
        strength: f32,
        target_height: f32,
    ) -> PromotedCommand {
        PromotedCommand::TerrainSculptFlatten {
            position,
            radius,
            strength,
            target_height,
        }
    }
}

/// Builder for material commands
pub struct MaterialCommandBuilder;

impl MaterialCommandBuilder {
    pub fn create(material_name: impl Into<String>) -> PromotedCommand {
        PromotedCommand::MaterialCreate {
            material_name: material_name.into(),
        }
    }

    pub fn delete(material_id: impl Into<String>) -> PromotedCommand {
        PromotedCommand::MaterialDelete {
            material_id: material_id.into(),
        }
    }

    pub fn bind_visual_response(
        material_id: impl Into<String>,
        visual_family: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::MaterialBindVisualResponse {
            material_id: material_id.into(),
            visual_family: visual_family.into(),
        }
    }

    pub fn bind_acoustic_profile(
        material_id: impl Into<String>,
        acoustic_profile: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::MaterialBindAcousticProfile {
            material_id: material_id.into(),
            acoustic_profile: acoustic_profile.into(),
        }
    }
}

/// Builder for audio commands
pub struct AudioCommandBuilder;

impl AudioCommandBuilder {
    pub fn create_source(source_name: impl Into<String>) -> PromotedCommand {
        PromotedCommand::AudioCreateSource {
            source_name: source_name.into(),
        }
    }

    pub fn bind_world_source(source_id: impl Into<String>, position: [f32; 3]) -> PromotedCommand {
        PromotedCommand::AudioBindWorldSource {
            source_id: source_id.into(),
            position,
        }
    }

    pub fn set_acoustic_profile(
        source_id: impl Into<String>,
        profile: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::AudioSetAcousticProfile {
            source_id: source_id.into(),
            profile: profile.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_command_builder() {
        let cmd = ProjectCommandBuilder::bootstrap("TestProject");
        assert_eq!(cmd.route_id(), "route.project.bootstrap.v1");

        let cmd = ProjectCommandBuilder::create("TestProject", "/root", "TestWorld");
        assert_eq!(cmd.route_id(), "route.project.create.v1");
    }

    #[test]
    fn test_runtime_command_builder() {
        let cmd = RuntimeCommandBuilder::play();
        assert_eq!(cmd.route_id(), "route.runtime.play.v1");

        let cmd = RuntimeCommandBuilder::pause();
        assert_eq!(cmd.route_id(), "route.runtime.pause.v1");
    }

    #[test]
    fn test_terrain_command_builder() {
        let cmd = TerrainCommandBuilder::sculpt_raise([0.0, 0.0], 1.0, 0.5);
        assert_eq!(cmd.route_id(), "route.terrain.sculpt.raise.v1");
    }
}
