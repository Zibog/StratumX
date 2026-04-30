use super::EditorApp;
use crate::editor_host::terrain_authoring_ops::TerrainTool;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

impl EditorApp {
    pub fn apply_terrain_tool(&mut self, position: [f32; 2]) {
        let brush = self.terrain_ops.brush_settings().clone();
        let current_tool = self.terrain_ops.current_tool();

        let command = match current_tool {
            TerrainTool::SculptRaise => Some(PromotedCommand::TerrainSculptRaise {
                position,
                radius: brush.radius,
                strength: brush.strength,
            }),
            TerrainTool::SculptLower => Some(PromotedCommand::TerrainSculptLower {
                position,
                radius: brush.radius,
                strength: brush.strength,
            }),
            TerrainTool::SculptSmooth => Some(PromotedCommand::TerrainSculptSmooth {
                position,
                radius: brush.radius,
                strength: brush.strength,
            }),
            TerrainTool::SculptFlatten => Some(PromotedCommand::TerrainSculptFlatten {
                position,
                radius: brush.radius,
                strength: brush.strength,
                target_height: brush.target_height,
            }),
            TerrainTool::PaintMaterial => Some(PromotedCommand::TerrainPaintMaterial {
                position,
                radius: brush.radius,
                strength: brush.strength,
                material_layer: brush.target_layer,
            }),
            TerrainTool::Select => None,
        };

        if let Some(command) = command {
            self.submit_promoted_command(command);
        }
    }
}
