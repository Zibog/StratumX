//! Command Spine Lifecycle Management
//!
//! This module manages the initialization and shutdown of the command spine,
//! including executor registry creation and cleanup operations.
//!
//! ## Responsibilities
//! - Initialize command spine with executor registry
//! - Create and register domain-specific executors
//! - Handle graceful shutdown and cleanup
//! - Build executor registry for command routing

use super::spine_core::CommandSpine;
use super::spine_routing::SpineRouting;

/// Initializes a new CommandSpine instance with full executor registry
///
/// # Returns
/// A fully initialized CommandSpine ready for command execution
pub fn initialize() -> CommandSpine {
    CommandSpine::new()
}

/// Shuts down the command spine and performs cleanup
///
/// # Arguments
/// * `spine` - The command spine instance to shut down
///
/// # Note
/// The current CommandSpine owns no external resources, so shutdown is a no-op cleanup seam.
pub fn shutdown(_spine: CommandSpine) {
    // CommandSpine currently tears down without additional cleanup work.
}

/// Builds the executor registry for command routing
///
/// # Returns
/// A SpineRouting instance with registered executors
///
/// # Note
/// The current lifecycle builds an empty routing registry and expands it as executors arrive.
pub fn build_registry() -> SpineRouting {
    SpineRouting::new()
}

/// Creates domain-specific executors for the command spine
///
/// # Returns
/// A vector of executor identifiers for registered executors
///
/// # Note
/// The current lifecycle exposes the intended domain slots before concrete executor instances exist.
pub fn create_executors() -> Vec<String> {
    vec![
        "world".to_string(),
        "runtime".to_string(),
        "terrain".to_string(),
        "environment".to_string(),
        "material".to_string(),
        "audio".to_string(),
        "build".to_string(),
        "shell".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_creates_spine() {
        let spine = initialize();
        assert!(spine.all_diagnostics().is_empty());
    }

    #[test]
    fn test_shutdown_completes_without_error() {
        let spine = initialize();
        shutdown(spine);
        // No panic means success
    }

    #[test]
    fn test_build_registry_creates_routing() {
        let _routing = build_registry();
        // Routing should be created successfully even before executor instances are attached.
    }

    #[test]
    fn test_create_executors_returns_all_domains() {
        let executors = create_executors();
        assert_eq!(executors.len(), 8);
        assert!(executors.contains(&"world".to_string()));
        assert!(executors.contains(&"runtime".to_string()));
        assert!(executors.contains(&"terrain".to_string()));
        assert!(executors.contains(&"environment".to_string()));
        assert!(executors.contains(&"material".to_string()));
        assert!(executors.contains(&"audio".to_string()));
        assert!(executors.contains(&"build".to_string()));
        assert!(executors.contains(&"shell".to_string()));
    }
}
