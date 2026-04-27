// Terrain Authority Container - Lifecycle operations

use super::types::{
    BrushState, TerrainAuthorityContainer, TerrainLifecycleState, TerrainRegistryState,
};

impl TerrainAuthorityContainer {
    /// Initialize the terrain authority
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.lifecycle_state != TerrainLifecycleState::Uninitialized {
            return Err(format!(
                "Cannot initialize: authority is in {:?} state",
                self.lifecycle_state
            ));
        }

        self.registry = TerrainRegistryState::default();
        self.brush_state = BrushState::default();
        self.lifecycle_state = TerrainLifecycleState::Initialized;

        Ok(())
    }

    /// Dispose the terrain authority
    pub fn dispose(&mut self) -> Result<(), String> {
        if self.lifecycle_state != TerrainLifecycleState::Initialized {
            return Err(format!(
                "Cannot dispose: authority is in {:?} state",
                self.lifecycle_state
            ));
        }

        self.registry = TerrainRegistryState::default();
        self.lifecycle_state = TerrainLifecycleState::Disposed;

        Ok(())
    }

    /// Get the current lifecycle state
    pub fn lifecycle_state(&self) -> TerrainLifecycleState {
        self.lifecycle_state
    }

    /// Check if the authority is initialized
    pub fn is_initialized(&self) -> bool {
        self.lifecycle_state == TerrainLifecycleState::Initialized
    }
}
