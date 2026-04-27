//! Audio Authority Container implementation modules.
//!
//! Contains the type definitions and split impl blocks for AudioAuthorityContainer.

mod lifecycle;
mod preview_connection;
mod profile_ops;
mod source_ops;
mod zone_ops;

/// Lifecycle state of the audio authority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioLifecycleState {
    Uninitialized,
    Initialized,
    Disposed,
}

/// Audio Authority Container
///
/// Owns the audio registry and manages its lifecycle.
/// Provides read-only query methods and routes all mutations through command spine.
pub struct AudioAuthorityContainer {
    /// Audio registry state (private - no direct mutation from outside)
    pub(super) registry: AudioRegistryState,
    /// Current lifecycle state
    pub(super) lifecycle_state: AudioLifecycleState,
    /// Preview connection to runtime kernel (if active)
    pub(super) preview_connection: Option<PreviewConnection>,
}

/// Audio registry state
#[derive(Default)]
pub struct AudioRegistryState {
    /// Available audio sources
    pub sources: Vec<AudioSource>,
    /// Available audio zones
    pub zones: Vec<AudioZone>,
    /// Acoustic profiles
    pub profiles: Vec<AcousticProfile>,
}

/// Audio source definition
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub id: String,
    pub name: String,
    pub source_type: AudioSourceType,
    pub active: bool,
}

/// Type of audio source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioSourceType {
    #[default]
    Ambient,
    Impact,
    Continuous,
    Trigger,
}

/// Audio zone definition
#[derive(Debug, Clone)]
pub struct AudioZone {
    pub id: String,
    pub name: String,
    pub bounds: [f32; 6], // [min_x, min_y, min_z, max_x, max_y, max_z]
    pub bound_sources: Vec<String>,
}

/// Acoustic profile
#[derive(Debug, Clone)]
pub struct AcousticProfile {
    pub id: String,
    pub name: String,
    pub absorption: f32,
    pub scattering: f32,
    pub reverb: f32,
    pub reverb_time: f32,
}

/// Preview connection to runtime kernel
#[derive(Debug, Clone)]
pub struct PreviewConnection {
    pub runtime_handle: String,
    pub session_id: String,
    pub active: bool,
}

impl AudioAuthorityContainer {
    /// Create a new AudioAuthorityContainer
    pub fn new() -> Self {
        Self {
            registry: AudioRegistryState::default(),
            lifecycle_state: AudioLifecycleState::Uninitialized,
            preview_connection: None,
        }
    }
}

impl Default for AudioAuthorityContainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_transitions() {
        let mut container = AudioAuthorityContainer::new();
        assert_eq!(
            container.lifecycle_state,
            AudioLifecycleState::Uninitialized
        );

        // Initialize
        container.initialize().expect("initialize should succeed");
        assert_eq!(container.lifecycle_state, AudioLifecycleState::Initialized);
        assert!(container.is_initialized());

        // Cannot initialize twice
        assert!(container.initialize().is_err());

        // Dispose
        container.dispose().expect("dispose should succeed");
        assert_eq!(container.lifecycle_state, AudioLifecycleState::Disposed);

        // Cannot dispose twice
        assert!(container.dispose().is_err());
    }

    #[test]
    fn test_preview_connection() {
        let mut container = AudioAuthorityContainer::new();
        container.initialize().expect("initialize should succeed");

        // Establish preview
        let conn = container
            .establish_preview_connection("runtime_0".to_string())
            .unwrap();
        assert!(container.is_preview_active());
        assert_eq!(conn.runtime_handle, "runtime_0");

        // Disconnect preview
        container.disconnect_preview().unwrap();
        assert!(!container.is_preview_active());
    }

    #[test]
    fn test_query_sources() {
        let mut container = AudioAuthorityContainer::new();
        container.initialize().expect("initialize should succeed");

        // Initially empty
        assert_eq!(container.query_sources().len(), 0);
    }

    #[test]
    fn test_initialization_with_preview_connection() {
        let mut container = AudioAuthorityContainer::new();

        // Initialize container
        container.initialize().expect("initialize should succeed");
        assert!(container.is_initialized());

        // Establish preview connection to runtime kernel
        let runtime_handle = "runtime_kernel_0".to_string();
        let conn = container
            .establish_preview_connection(runtime_handle.clone())
            .expect("preview connection should succeed");

        // Verify preview connection is active
        assert!(container.is_preview_active());
        assert_eq!(conn.runtime_handle, runtime_handle);

        // Verify container state
        assert_eq!(container.lifecycle_state, AudioLifecycleState::Initialized);
        assert!(container.preview_connection.is_some());
    }

    #[test]
    fn test_dispose_disconnects_preview_and_finalizes_registry() {
        let mut container = AudioAuthorityContainer::new();

        // Initialize container
        container.initialize().expect("initialize should succeed");

        // Establish preview connection
        container
            .establish_preview_connection("runtime_0".to_string())
            .expect("preview connection should succeed");
        assert!(container.is_preview_active());

        // Dispose should disconnect preview and finalize registry
        container.dispose().expect("dispose should succeed");

        // Verify preview is disconnected
        assert!(!container.is_preview_active());
        assert!(container.preview_connection.is_none());

        // Verify registry is cleared (finalized)
        assert_eq!(container.registry.sources.len(), 0);
        assert_eq!(container.registry.zones.len(), 0);
        assert_eq!(container.registry.profiles.len(), 0);

        // Verify lifecycle state is Disposed
        assert_eq!(container.lifecycle_state, AudioLifecycleState::Disposed);
    }
}
