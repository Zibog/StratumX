//! Open world dialog driven by shell-owned dialog state.

use eframe::egui;

/// State for the open world dialog
#[derive(Debug, Clone, Default)]
pub struct OpenWorldDialog {
    pub is_open: bool,
    pub world_path: String,
}

/// Validation tone for UI feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationTone {
    Success,
    Warning,
    Error,
}

/// Validation result message
#[derive(Debug, Clone)]
pub struct ValidationMessage {
    pub tone: ValidationTone,
    pub message: String,
}

/// Callback type for file dialog - returns selected path
pub type FileDialogCallback = Box<dyn FnMut(&str) -> Option<std::path::PathBuf>>;

/// Callback type for opening a world
pub type OpenWorldCallback = Box<dyn FnMut(&str) -> Result<(), String>>;

/// Open world dialog widget
pub struct OpenWorldDialogPanel {
    pub state: OpenWorldDialog,
    pub on_browse: Option<FileDialogCallback>,
    pub on_open: Option<OpenWorldCallback>,
    pub on_cancel: Option<Box<dyn FnMut()>>,
    pub validation: Option<ValidationMessage>,
}

impl OpenWorldDialogPanel {
    pub fn new() -> Self {
        Self {
            state: OpenWorldDialog::default(),
            on_browse: None,
            on_open: None,
            on_cancel: None,
            validation: None,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.state.is_open {
            return;
        }

        egui::Window::new("Open World")
            .collapsible(true)
            .resizable(true)
            .default_width(600.0)
            .show(ctx, |ui| {
                ui.label("Enter world package path (directory containing world.json):");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.state.world_path);
                    if ui.button("Browse...").clicked() {
                        if let Some(callback) = &mut self.on_browse {
                            if let Some(path) = callback(&self.state.world_path) {
                                self.state.world_path = path.to_string_lossy().to_string();
                            }
                        }
                    }
                });

                ui.separator();

                if let Some(validation) = &self.validation {
                    let color = match validation.tone {
                        ValidationTone::Success => egui::Color32::GREEN,
                        ValidationTone::Warning => egui::Color32::YELLOW,
                        ValidationTone::Error => egui::Color32::RED,
                    };
                    ui.colored_label(color, &validation.message);
                }

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Open").clicked() {
                        if let Some(callback) = &mut self.on_open {
                            if let Err(error) = callback(&self.state.world_path) {
                                self.validation = Some(ValidationMessage {
                                    tone: ValidationTone::Error,
                                    message: error,
                                });
                            }
                        }
                    }

                    if ui.button("Cancel").clicked() {
                        self.state.is_open = false;
                        self.state.world_path.clear();
                        if let Some(callback) = &mut self.on_cancel {
                            callback();
                        }
                    }
                });

                ui.separator();
                ui.label("World package must contain world.json");
                ui.label("Example: /path/to/my_world/");
                ui.label("Package includes: world.json, terrain/, environment/");
            });
    }
}
