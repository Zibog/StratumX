// Command builder modules

pub mod assets;
pub mod audio;
pub mod build_release;
pub mod diagnostics;
pub mod graphics;
pub mod materials;
pub mod world;

// Re-export all builders for backward compatibility
pub use audio::*;
pub use build_release::*;
pub use materials::*;
pub use world::*;

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
