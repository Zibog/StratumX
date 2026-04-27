// Material Authority Container
//
// Provides controlled access to material authority views using SDK types.
// This container works with view types from the SDK layer, avoiding
// direct dependencies on editor implementation types.

use editor_dto_law::{MaterialProfileView, MaterialRegistryView, ObjectHandle};

/// Material Authority Container
///
/// Provides access to material authority views using SDK types.
/// Works with view types from the SDK layer, avoiding direct dependencies
/// on editor implementation types. All mutations are routed through command spine.
pub struct MaterialAuthorityContainer {
    registry_view: MaterialRegistryView,
    lifecycle_state: LifecycleState,
}

/// Lifecycle state of the material authority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Uninitialized,
    Initialized,
    Disposed,
}

impl MaterialAuthorityContainer {
    /// Create a new MaterialAuthorityContainer
    /// The authority starts in Uninitialized state
    pub fn new() -> Self {
        Self {
            registry_view: MaterialRegistryView::empty(),
            lifecycle_state: LifecycleState::Uninitialized,
        }
    }

    /// Initialize the material authority
    /// Transitions from Uninitialized to Initialized state
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.lifecycle_state != LifecycleState::Uninitialized {
            return Err(format!(
                "Cannot initialize: authority is in {:?} state",
                self.lifecycle_state
            ));
        }

        // Initialize with empty registry view
        self.registry_view = MaterialRegistryView::empty();
        self.lifecycle_state = LifecycleState::Initialized;

        Ok(())
    }

    /// Dispose the material authority
    /// Finalizes the registry and flushes pending changes
    /// Transitions from Initialized to Disposed state
    pub fn dispose(&mut self) -> Result<(), String> {
        if self.lifecycle_state != LifecycleState::Initialized {
            return Err(format!(
                "Cannot dispose: authority is in {:?} state",
                self.lifecycle_state
            ));
        }

        // Finalize the registry (flush pending changes, cleanup)
        // In a full implementation, this would propagate changes to world truth
        self.lifecycle_state = LifecycleState::Disposed;

        Ok(())
    }

    /// Query material profiles (read-only)
    pub fn query_profiles(&self) -> Vec<&MaterialProfileView> {
        self.registry_view.materials.iter().collect()
    }

    /// Query a specific material profile by handle (read-only)
    pub fn query_profile(&self, handle: ObjectHandle) -> Option<&MaterialProfileView> {
        self.registry_view
            .materials
            .iter()
            .find(|profile| profile.handle == handle)
    }

    /// Get the current lifecycle state
    pub fn lifecycle_state(&self) -> LifecycleState {
        self.lifecycle_state
    }

    /// Check if the authority is initialized and ready for use
    pub fn is_initialized(&self) -> bool {
        self.lifecycle_state == LifecycleState::Initialized
    }
}

impl Default for MaterialAuthorityContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_transitions() {
        let mut container = MaterialAuthorityContainer::new();
        assert_eq!(container.lifecycle_state(), LifecycleState::Uninitialized);

        // Initialize
        container.initialize().expect("initialize should succeed");
        assert_eq!(container.lifecycle_state(), LifecycleState::Initialized);
        assert!(container.is_initialized());

        // Cannot initialize twice
        assert!(container.initialize().is_err());

        // Dispose
        container.dispose().expect("dispose should succeed");
        assert_eq!(container.lifecycle_state(), LifecycleState::Disposed);

        // Cannot dispose twice
        assert!(container.dispose().is_err());
    }

    #[test]
    fn test_query_profiles() {
        let mut container = MaterialAuthorityContainer::new();
        container.initialize().expect("initialize should succeed");

        // Initially empty
        assert_eq!(container.query_profiles().len(), 0);

        // Note: Profile creation should be done through command spine in production
        // This test just verifies the query interface works
    }
}
