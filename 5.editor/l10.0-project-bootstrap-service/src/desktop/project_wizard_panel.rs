//! Minimal project wizard for the launch contour.

use eframe::egui;

/// State for the new project dialog
#[derive(Debug, Clone, Default)]
pub struct NewProjectDialog {
    pub is_open: bool,
    pub project_name: String,
    pub project_path: String,
    pub world_name: String,
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

/// Callback type for project creation
pub type CreateProjectCallback = Box<dyn FnMut(&str, &str, &str) -> Result<(), String>>;

/// Project wizard panel widget
pub struct ProjectWizardPanel {
    pub state: NewProjectDialog,
    pub on_browse: Option<FileDialogCallback>,
    pub on_create: Option<CreateProjectCallback>,
    pub on_cancel: Option<Box<dyn FnMut()>>,
    pub validation: Option<ValidationMessage>,
}

impl ProjectWizardPanel {
    pub fn new() -> Self {
        Self {
            state: NewProjectDialog::default(),
            on_browse: None,
            on_create: None,
            on_cancel: None,
            validation: None,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.state.is_open {
            return;
        }

        egui::Window::new("New Project")
            .collapsible(true)
            .resizable(true)
            .default_width(500.0)
            .show(ctx, |ui| {
                ui.label("Project Name");
                ui.text_edit_singleline(&mut self.state.project_name);

                ui.label("Project Root Directory");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.state.project_path);
                    if ui.button("Browse...").clicked() {
                        if let Some(callback) = &mut self.on_browse {
                            if let Some(path) = callback(&self.state.project_path) {
                                self.state.project_path = path.to_string_lossy().to_string();
                            }
                        }
                    }
                });

                if let Some(validation) = &self.validation {
                    let color = match validation.tone {
                        ValidationTone::Success => egui::Color32::GREEN,
                        ValidationTone::Warning => egui::Color32::YELLOW,
                        ValidationTone::Error => egui::Color32::RED,
                    };
                    ui.colored_label(color, &validation.message);
                }

                ui.label("World Name");
                ui.text_edit_singleline(&mut self.state.world_name);

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        if let Some(callback) = &mut self.on_create {
                            if let Err(error) = callback(
                                &self.state.project_name,
                                &self.state.project_path,
                                &self.state.world_name,
                            ) {
                                self.validation = Some(ValidationMessage {
                                    tone: ValidationTone::Error,
                                    message: error,
                                });
                            }
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.state.is_open = false;
                        if let Some(callback) = &mut self.on_cancel {
                            callback();
                        }
                    }
                });

                ui.separator();
                ui.label("Creates: {root}/{project_name}/worlds/{world_name}/world.json");
            });
    }
}
