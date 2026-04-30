// World command builders

use crate::promoted_commands::types::PromotedCommand;

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
