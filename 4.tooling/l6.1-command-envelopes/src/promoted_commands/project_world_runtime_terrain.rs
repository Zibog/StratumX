use serde::{Deserialize, Serialize};

/// Project lifecycle commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectCommand {
    Bootstrap {
        project_name: String,
    },
    Create {
        project_name: String,
        project_root: String,
        world_name: String,
    },
    Save {
        save_path: String,
    },
    Build {
        target_platform: String,
    },
    Export {
        export_path: String,
    },
    Launch {
        launch_mode: String,
    },
    VerifyFirstResult,
}

/// World lifecycle commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldCommand {
    Open { world_path: String },
    Save { world_path: String },
    Close,
}

/// Runtime control commands
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RuntimeCommand {
    Play,
    Pause,
    Stop,
    Simulate,
}

/// Terrain authoring commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrainCommand {
    Import {
        heightmap_path: String,
    },
    Rebuild,
    SculptRaise {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
    SculptLower {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
    SculptSmooth {
        position: [f32; 2],
        radius: f32,
        strength: f32,
    },
    SculptFlatten {
        position: [f32; 2],
        radius: f32,
        strength: f32,
        target_height: f32,
    },
    PaintMaterial {
        position: [f32; 2],
        radius: f32,
        strength: f32,
        material_layer: u32,
    },
    SetLayerMaterial {
        layer_id: u16,
        albedo_texture_path: String,
        uv_scale: [f32; 2],
    },
    AddHole {
        position: [f32; 2],
        radius: f32,
    },
    RemoveHole {
        position: [f32; 2],
        radius: f32,
    },
}
