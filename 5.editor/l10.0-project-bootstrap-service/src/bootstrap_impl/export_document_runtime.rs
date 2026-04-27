use crate::*;

impl EditorProduct {
    pub fn export_document(&self) -> EditorProjectDocument {
        let snapshot = self.tooling.snapshot();
        EditorProjectDocument {
            title: self.title.clone(),
            project_name: self.project_name.clone(),
            viewport: self.viewport.clone(),
            tool_context: self.tool_context.clone(),
            workspace_layout: self.workspace_layout.clone(),
            interaction_routing: self.interaction_routing.clone(),
            plugin_host: self.plugin_host.clone(),
            package_service: self.package_service.clone(),
            collaboration_surface: self.collaboration_surface.clone(),
            review_surface: self.review_surface.clone(),
            approval_surface: self.approval_surface.clone(),
            playtest_surface: self.playtest_surface.clone(),
            production_surface: self.production_surface.clone(),
            learning_surface: self.learning_surface.clone(),
            assistant_surface: self.assistant_surface.clone(),
            objects: snapshot.objects.clone(),
        }
    }
    pub fn save_project_to_path(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let document = self.export_document();
        let data = serde_json::to_string_pretty(&document).map_err(|err| err.to_string())?;
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|err| err.to_string())?;
            }
        }
        fs::write(path, data).map_err(|err| err.to_string())
    }
}
