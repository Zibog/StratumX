//! Launch-critical terrain panel.
use super::EditorApp;
use crate::{editor_actions::ActionPayload, editor_host::terrain_authoring_ops::TerrainTool};
use eframe::egui;
use stratumx_editor_l8_0_editor_shell::{status_bar::MessageType, FileDialogKind};
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

impl EditorApp {
    pub fn render_terrain_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("terrain_panel")
            .min_width(280.0)
            .max_width(340.0)
            .show(ctx, |ui| {
                ui.heading("Terrain");
                ui.separator();

                self.render_terrain_tools(ui);
                ui.separator();
                self.render_brush_settings(ui);
                ui.separator();
                self.render_terrain_info(ui);
                ui.separator();
                self.render_terrain_actions(ui);
            });
    }

    fn render_terrain_tools(&mut self, ui: &mut egui::Ui) {
        let tools = [
            ("Select", TerrainTool::Select),
            ("Raise", TerrainTool::SculptRaise),
            ("Lower", TerrainTool::SculptLower),
            ("Smooth", TerrainTool::SculptSmooth),
            ("Flatten", TerrainTool::SculptFlatten),
            ("Paint", TerrainTool::PaintMaterial),
        ];

        ui.label("Tools");
        ui.horizontal_wrapped(|ui| {
            for (label, tool) in tools {
                let selected = self.terrain_ops.current_tool() == tool;
                if ui.selectable_label(selected, label).clicked() {
                    self.terrain_ops.set_tool(tool);
                }
            }
        });
    }

    fn render_brush_settings(&mut self, ui: &mut egui::Ui) {
        ui.label("Brush");
        let current_tool = self.terrain_ops.current_tool();
        let brush = self.terrain_ops.brush_settings_mut();

        ui.add(egui::Slider::new(&mut brush.radius, 0.5..=500.0).text("Radius"));
        ui.add(egui::Slider::new(&mut brush.strength, 0.0..=1.0).text("Strength"));

        if current_tool == TerrainTool::PaintMaterial {
            ui.add(egui::Slider::new(&mut brush.target_layer, 0..=3).text("Layer"));
        }

        if current_tool == TerrainTool::SculptFlatten {
            ui.add(egui::Slider::new(&mut brush.target_height, 0.0..=1000.0).text("Target Height"));
        }
    }

    fn render_terrain_info(&mut self, ui: &mut egui::Ui) {
        ui.label("Active Terrain");
        if let Some(summary) = self.world_summary() {
            ui.label(format!(
                "Resolution: {}x{}",
                summary.terrain.resolution[0], summary.terrain.resolution[1]
            ));
            ui.label(format!(
                "World Size: {:.0} x {:.0}",
                summary.terrain.world_size[0], summary.terrain.world_size[1]
            ));
            ui.label(format!(
                "Chunks: {}x{}",
                summary.terrain.chunk_grid[0], summary.terrain.chunk_grid[1]
            ));
            ui.label(format!("Layers: {}", summary.terrain.layer_count));
        } else {
            ui.label("No world loaded");
        }
    }

    fn render_terrain_actions(&mut self, ui: &mut egui::Ui) {
        if ui.button("Rebuild Mesh").clicked() {
            self.submit_promoted_command(PromotedCommand::TerrainRebuild);
        }

        ui.separator();
        ui.label("Heightmap Import");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.state.terrain_heightmap_input);
            if ui.button("Browse…").clicked() {
                if let Some(path) = self
                    .state
                    .shell
                    .show_file_dialog(FileDialogKind::HeightmapFile)
                {
                    self.state.terrain_heightmap_input = path.to_string_lossy().to_string();
                }
            }
        });
        if ui.button("Import Heightmap").clicked() {
            let path = self.state.terrain_heightmap_input.trim();
            if path.is_empty() {
                self.shell_status("Enter a heightmap path first", MessageType::Warning);
                return;
            }
            self.dispatch_phase4_action(
                "terrain.import_heightmap",
                Some(ActionPayload::String(path.to_string())),
            );
        }

        ui.separator();
        ui.label("Layer Materials");
        self.render_terrain_layer_materials(ui);
    }

    fn render_terrain_layer_materials(&mut self, ui: &mut egui::Ui) {
        for layer_id in 0..2u16 {
            ui.push_id(layer_id, |ui| {
                ui.label(format!("Layer {}", layer_id));
                let current_path = if layer_id == 0 {
                    &self.state.terrain_layer0_albedo_input
                } else {
                    &self.state.terrain_layer1_albedo_input
                };

                let mut path_buf = current_path.clone();
                ui.text_edit_singleline(&mut path_buf);

                if ui.button("Browse…").clicked() {
                    if let Some(path) = self
                        .state
                        .shell
                        .show_file_dialog(FileDialogKind::TextureFile)
                    {
                        path_buf = path.to_string_lossy().to_string();
                    }
                }

                if layer_id == 0 {
                    self.state.terrain_layer0_albedo_input = path_buf.clone();
                } else {
                    self.state.terrain_layer1_albedo_input = path_buf.clone();
                }

                if !path_buf.is_empty() && ui.button("Apply").clicked() {
                    self.submit_promoted_command(PromotedCommand::TerrainSetLayerMaterial {
                        layer_id,
                        albedo_texture_path: path_buf.clone(),
                        uv_scale: [8.0, 8.0],
                    });
                }
            });
        }
    }

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
