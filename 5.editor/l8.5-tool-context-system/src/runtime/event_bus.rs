//! Event Bus Infrastructure
//!
//! Provides event emission and subscription for domain services to communicate
//! state changes without direct coupling.

use crate::{CacheId, Severity, StateId};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Event bus trait for emitting and subscribing to editor events
pub trait EventBus: Send + Sync {
    /// Emit an event to all subscribers
    fn emit(&self, event: EditorEvent);

    /// Subscribe to events with a handler function
    fn subscribe(&self, handler: Box<dyn Fn(&EditorEvent) + Send + Sync>);
}

/// Editor events emitted by domain services
#[derive(Debug, Clone, PartialEq)]
pub enum EditorEvent {
    // State change events
    StateChanged {
        state_id: StateId,
        change_type: ChangeType,
    },

    // Cache events
    CacheInvalidated {
        cache_id: CacheId,
    },
    CacheRebuilt {
        cache_id: CacheId,
    },

    // Service events - World Session
    WorldOpened {
        world_id: Uuid,
    },
    WorldClosed,
    WorldSaved {
        generation: u64,
    },

    // Service events - Terrain Authoring
    TerrainModified {
        region: String,
    },
    TerrainLayerChanged {
        layer_name: String,
    },

    // Service events - Material Authoring
    MaterialProfileCreated {
        profile_id: Uuid,
    },
    MaterialBound {
        entity_id: u32,
        profile_id: Uuid,
    },

    // Service events - Audio Authoring
    AudioSourcePlaced {
        source_id: Uuid,
    },
    AudioProfileUpdated {
        profile_id: Uuid,
    },

    // Service events - Environment Authoring
    SkyConfigured,
    WeatherChanged,
    LightingUpdated,

    // Service events - Runtime Mode
    PreviewModeEntered,
    PreviewModeExited,
    RuntimeInitialized,
    SimulationStarted,
    SimulationStopped,

    // Diagnostic events
    DiagnosticAdded {
        severity: Severity,
        message: String,
    },
    DiagnosticsCleared,
}

/// Type of state change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
}

/// Basic event bus implementation with synchronous dispatch
pub struct BasicEventBus {
    handlers: Arc<Mutex<Vec<Box<dyn Fn(&EditorEvent) + Send + Sync>>>>,
}

impl BasicEventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for BasicEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus for BasicEventBus {
    fn emit(&self, event: EditorEvent) {
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler(&event);
        }
    }

    fn subscribe(&self, handler: Box<dyn Fn(&EditorEvent) + Send + Sync>) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(handler);
    }
}


