//! Preview connection operations for audio authority.

use super::{AudioAuthorityContainer, AudioLifecycleState, PreviewConnection};

impl AudioAuthorityContainer {
    /// Establish preview connection to runtime kernel
    pub fn establish_preview_connection(
        &mut self,
        runtime_handle: String,
    ) -> Result<PreviewConnection, String> {
        if self.lifecycle_state != AudioLifecycleState::Initialized {
            return Err("Authority not initialized".to_string());
        }

        let connection = PreviewConnection {
            runtime_handle,
            session_id: uuid::Uuid::new_v4().to_string(),
            active: true,
        };

        self.preview_connection = Some(connection.clone());
        Ok(connection)
    }

    /// Disconnect preview connection
    pub fn disconnect_preview(&mut self) -> Result<(), String> {
        if let Some(ref mut conn) = self.preview_connection {
            conn.active = false;
        }
        self.preview_connection = None;
        Ok(())
    }

    /// Check if preview is active
    pub fn is_preview_active(&self) -> bool {
        self.preview_connection
            .as_ref()
            .map(|c| c.active)
            .unwrap_or(false)
    }

    /// Get preview connection info
    pub fn preview_connection(&self) -> Option<&PreviewConnection> {
        self.preview_connection.as_ref()
    }
}
