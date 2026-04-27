//! EditorHost - Thin Facade
//!
//! This module provides the EditorHost struct that acts as a thin facade over
//! the editor's state containers and domain services. EditorHost contains only
//! lifecycle coordination logic and delegates all business operations to domain services.

use crate::{CacheLayer, EditorServices, OwnershipViolation, QueryLayer, StateContainerSystem};
use std::sync::{Arc, Mutex};

/// EditorHost - Thin facade for editor coordination
///
/// EditorHost is responsible for:
/// - Lifecycle coordination (initialize, shutdown)
/// - Service delegation
/// - State ownership validation
///
/// EditorHost does NOT:
/// - Contain business logic
/// - Directly manipulate state containers
/// - Implement domain-specific operations
pub struct EditorHost {
    state_system: Arc<StateContainerSystem>,
    query_layer: QueryLayer,
    cache_layer: Arc<Mutex<CacheLayer>>,
    services: EditorServices,
    initialized: bool,
}

impl EditorHost {
    /// Create a new EditorHost instance
    ///
    /// # Arguments
    ///
    /// * `state_system` - State container system
    /// * `query_layer` - Query layer for read-only queries
    /// * `cache_layer` - Cache layer for derived state
    /// * `services` - Editor services
    ///
    /// # Returns
    ///
    /// EditorHost instance
    pub fn new(
        state_system: Arc<StateContainerSystem>,
        query_layer: QueryLayer,
        cache_layer: Arc<Mutex<CacheLayer>>,
        services: EditorServices,
    ) -> Self {
        Self {
            state_system,
            query_layer,
            cache_layer,
            services,
            initialized: false,
        }
    }

    /// Initialize the editor host
    ///
    /// Performs lifecycle initialization including:
    /// - State ownership validation
    /// - Service initialization
    /// - Cache warming (if needed)
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("EditorHost already initialized".to_string());
        }

        // Validate state ownership
        self.validate_state_ownership().map_err(|violations| {
            format!(
                "State ownership validation failed: {} violations",
                violations.len()
            )
        })?;

        self.initialized = true;
        Ok(())
    }

    /// Shutdown the editor host
    ///
    /// Performs cleanup and resource release
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Err("EditorHost not initialized".to_string());
        }

        // Clear caches
        {
            let cache = self.cache_layer.lock().unwrap();
            cache.clear_all();
        }

        self.initialized = false;
        Ok(())
    }

    /// Get immutable reference to services
    pub fn services(&self) -> &EditorServices {
        &self.services
    }

    /// Get mutable reference to services
    pub fn services_mut(&mut self) -> &mut EditorServices {
        &mut self.services
    }

    /// Get reference to query layer
    pub fn query(&self) -> &QueryLayer {
        &self.query_layer
    }

    /// Get reference to cache layer
    pub fn cache(&self) -> &Arc<Mutex<CacheLayer>> {
        &self.cache_layer
    }

    /// Validate state ownership uniqueness
    ///
    /// Ensures that all state has exactly one owner and no ownership violations exist
    pub fn validate_state_ownership(&self) -> Result<(), Vec<OwnershipViolation>> {
        self.state_system.validate_ownership_uniqueness()
    }

    /// Check if editor host is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

