// App Actions - Command Spine Integration
//
// This module provides the canonical command routing layer between UI and EditorHost.
// All UI interactions MUST go through this layer, never calling host.* directly.
//
// Architecture:
// UI → dispatch_action() → PromotedCommand → CommandSpine → EditorHost → World/Terrain/Environment
//
// This enforces:
// - Single source of truth for all editor operations
// - Traceable command history
// - Undo/redo capability foundation
// - Diagnostic integration
// - Canonical route IDs

pub mod audio_actions;
pub mod build_actions;
pub mod environment_actions;
pub mod material_actions;
pub mod runtime_actions;
pub mod terrain_actions;
pub mod world_actions;

use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

/// Action dispatcher - converts action IDs to PromotedCommands
pub struct ActionDispatcher;

impl ActionDispatcher {
    /// Dispatches an action by ID with optional payload
    pub fn dispatch(action_id: &str, payload: Option<ActionPayload>) -> Option<PromotedCommand> {
        match action_id {
            // File operations
            "file.open_world" => world_actions::open_world(payload),
            "file.save" => world_actions::save_current_world(payload),
            "file.close_world" => world_actions::close_world(payload),
            "file.new_project" => None, // Opens wizard, not a command
            "file.import" => None,      // Opens import panel
            "file.export" => None,      // Opens export panel

            // World operations
            "world.play" => runtime_actions::play(payload),
            "world.pause" => runtime_actions::pause(payload),
            "world.stop" => runtime_actions::stop(payload),
            "world.simulate" => runtime_actions::simulate(payload),

            // Build operations
            "build.run" => Some(PromotedCommand::BuildRun),
            "build.release" => Some(PromotedCommand::BuildRelease),

            // Automation operations
            "automation.validate" => Some(PromotedCommand::ValidationRunFull),
            "automation.rebuild_all" => Some(PromotedCommand::AutomationRebuildAll),

            // Quality operations
            "quality.smoke" => Some(PromotedCommand::ValidationRunSmoke),
            "quality.gates" => None, // Opens quality panel

            // Terrain operations
            "terrain.import_heightmap" => terrain_actions::import_heightmap(payload),
            "terrain.rebuild" => terrain_actions::rebuild(payload),
            "terrain.sculpt_raise" => terrain_actions::sculpt_raise(payload),
            "terrain.sculpt_lower" => terrain_actions::sculpt_lower(payload),
            "terrain.sculpt_smooth" => terrain_actions::sculpt_smooth(payload),
            "terrain.sculpt_flatten" => terrain_actions::sculpt_flatten(payload),
            "terrain.paint_material" => terrain_actions::paint_material(payload),
            "terrain.add_hole" => terrain_actions::add_hole(payload),
            "terrain.remove_hole" => terrain_actions::remove_hole(payload),

            // Environment operations
            "environment.set_time" => environment_actions::set_time_of_day(payload),
            "environment.set_weather" => environment_actions::set_weather_regime(payload),
            "environment.set_cloud_coverage" => environment_actions::set_cloud_coverage(payload),
            "environment.set_fog_density" => environment_actions::set_fog_density(payload),

            _ => None,
        }
    }
}

/// Action payload - generic data container for action parameters
#[derive(Debug, Clone)]
pub enum ActionPayload {
    String(String),
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    U32(u32),
    Multiple(Vec<ActionPayload>),
    TerrainOp {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
}

impl ActionPayload {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ActionPayload::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            ActionPayload::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_vec2(&self) -> Option<[f32; 2]> {
        match self {
            ActionPayload::Vec2(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<[f32; 3]> {
        match self {
            ActionPayload::Vec3(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            ActionPayload::U32(u) => Some(*u),
            _ => None,
        }
    }
}
