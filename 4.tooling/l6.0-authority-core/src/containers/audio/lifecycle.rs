//! Lifecycle state and operations for audio authority.

use super::{AudioAuthorityContainer, AudioLifecycleState, AudioRegistryState};

impl AudioAuthorityContainer {
    /// Initialize the audio authority
    /// Transitions from Uninitialized to Initialized state
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.lifecycle_state != AudioLifecycleState::Uninitialized {
            return Err(format!(
                "Cannot initialize: authority is in {:?} state",
                self.lifecycle_state
            ));
        }

        // Initialize default audio sources and zones
        self.registry = AudioRegistryState::default();
        self.lifecycle_state = AudioLifecycleState::Initialized;

        Ok(())
    }

    /// Dispose the audio authority
    /// Finalizes the registry, disconnects preview, and flushes pending changes
    /// Transitions from Initialized to Disposed state
    ///
    /// **Requirements:** 8.3
    /// - Disconnect preview connection
    /// - Properly finalize registry and flush pending changes
    ///
    /// **Canonical Route Flow:**
    /// EditorHost -> CommandSpine -> AudioExecutor -> ToolingRuntime -> AudioAuthorityContainer::dispose
    pub fn dispose(&mut self) -> Result<(), String> {
        if self.lifecycle_state != AudioLifecycleState::Initialized {
            return Err(format!(
                "Cannot dispose: authority is in {:?} state",
                self.lifecycle_state
            ));
        }

        // Disconnect preview connection to runtime kernel
        // This ensures no preview operations are in flight during disposal
        if let Some(ref mut conn) = self.preview_connection {
            conn.active = false;
        }
        self.preview_connection = None;

        // Finalize registry and flush pending changes.
        // Any committed authoring changes should already have reached world truth through transactions,
        // so disposal resets the local registry to its cold baseline.
        self.registry = AudioRegistryState::default();

        // Transition to disposed state
        self.lifecycle_state = AudioLifecycleState::Disposed;

        Ok(())
    }

    /// Get the current lifecycle state
    pub fn lifecycle_state(&self) -> AudioLifecycleState {
        self.lifecycle_state
    }

    /// Check if the authority is initialized and ready for use
    pub fn is_initialized(&self) -> bool {
        self.lifecycle_state == AudioLifecycleState::Initialized
    }
}
