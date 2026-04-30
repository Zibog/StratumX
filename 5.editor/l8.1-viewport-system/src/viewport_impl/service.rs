//! Viewport Service Implementation

use serde::{Deserialize, Serialize};
use stratumx_tooling::{ToolingError, ToolingRuntime};

use super::types::{CameraTransform, InspectorSystem, SceneId, ViewportData, ViewportSystem};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    pub viewport: ViewportSystem,
    pub inspector: InspectorSystem,
    pub tooling: ToolingRuntime,
}

impl ViewportSystem {
    pub(crate) fn validate_active_viewport(&self) -> Result<(), ToolingError> {
        if self.active_viewport > 3 {
            return Err(ToolingError::Message(format!(
                "active viewport index {} is out of range",
                self.active_viewport
            )));
        }
        Ok(())
    }

    pub(crate) fn camera_bookmark_from_transform(camera: &CameraTransform) -> String {
        format!(
            "cam:{:.2},{:.2},{:.2}->{:.2},{:.2},{:.2}",
            camera.position[0],
            camera.position[1],
            camera.position[2],
            camera.look_at[0],
            camera.look_at[1],
            camera.look_at[2]
        )
    }

    pub fn bind_to_scene(&mut self, scene_id: SceneId) -> Result<(), ToolingError> {
        if scene_id.0.trim().is_empty() {
            return Err(ToolingError::Message(
                "scene binding cannot be empty".into(),
            ));
        }
        self.scene_binding = Some(scene_id);
        Ok(())
    }

    pub fn update_camera(&mut self, camera_transform: CameraTransform) -> Result<(), ToolingError> {
        let all_components = camera_transform
            .position
            .into_iter()
            .chain(camera_transform.look_at)
            .chain([camera_transform.fov_deg]);
        if all_components.into_iter().any(|value| !value.is_finite()) {
            return Err(ToolingError::Message(
                "camera transform contains non-finite values".into(),
            ));
        }
        if camera_transform.fov_deg <= 0.0 {
            return Err(ToolingError::Message(
                "camera field of view must be positive".into(),
            ));
        }
        self.camera_bookmark = Self::camera_bookmark_from_transform(&camera_transform);
        self.camera = camera_transform;
        Ok(())
    }
}

impl EditorProduct {
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        match self.extract_viewport_data() {
            Ok(data) => {
                self.viewport.selected = data.selected.clone();
                self.inspector.selected = data.selected.first().copied();
                self.viewport.camera_bookmark = data.camera_bookmark.clone();
                self.viewport.last_valid_data = Some(data);
                self.viewport.last_sync_error = None;
                Ok(())
            }
            Err(error) => {
                self.viewport.last_sync_error = Some(error.to_string());
                if let Some(snapshot) = self.viewport.last_valid_data.clone() {
                    self.viewport.selected = snapshot.selected.clone();
                    self.inspector.selected = snapshot.selected.first().copied();
                    self.viewport.camera_bookmark = snapshot.camera_bookmark;
                }
                Err(error)
            }
        }
    }

    pub fn extract_viewport_data(&self) -> Result<ViewportData, ToolingError> {
        self.viewport.validate_active_viewport()?;

        let selected = self.tooling.queued_selection().to_vec();
        let object_count = self.tooling.objects().len();
        let camera_bookmark = selected
            .first()
            .and_then(|handle| self.tooling.objects().get(handle))
            .map(|object| format!("focus:{}", object.label))
            .filter(|bookmark| !bookmark.is_empty())
            .or_else(|| {
                (!self.viewport.camera_bookmark.is_empty())
                    .then(|| self.viewport.camera_bookmark.clone())
            })
            .unwrap_or_else(|| {
                ViewportSystem::camera_bookmark_from_transform(&self.viewport.camera)
            });

        Ok(ViewportData {
            active_viewport: self.viewport.active_viewport,
            projection: self.viewport.projection,
            selected,
            object_count,
            active_scene: self.viewport.scene_binding.clone(),
            camera_bookmark,
        })
    }

    pub fn bind_to_scene(&mut self, scene_id: SceneId) -> Result<(), ToolingError> {
        self.viewport.bind_to_scene(scene_id)?;
        self.refresh_from_tooling()
    }

    pub fn update_camera(&mut self, camera_transform: CameraTransform) -> Result<(), ToolingError> {
        self.viewport.update_camera(camera_transform)
    }

    pub fn last_sync_error(&self) -> Option<&str> {
        self.viewport.last_sync_error.as_deref()
    }

    pub fn select_object(
        &mut self,
        handle: stratumx_tooling::ObjectHandle,
    ) -> Result<(), ToolingError> {
        self.viewport.selected = vec![handle];
        self.inspector.selected = Some(handle);
        self.tooling.queue_selection(vec![handle]);
        self.refresh_from_tooling()
    }
}
