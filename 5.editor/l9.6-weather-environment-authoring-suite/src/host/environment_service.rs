//! Environment Service
//!
//! Manages sky/weather/time operations through canonical command spine routing.
//! All operations emit environment_changed events.
//!
//! **Requirements: 12.4, 12.8, 29.2, 29.3**

use super::environment_types::*;
use super::event_bus_trait::{EventBusInterface, ServiceEvent};
use std::sync::Arc;

/// Environment service - handles sky/weather/time operations
///
/// **Canonical Route Flow:**
/// UI → Action → Command → CommandSpine → EnvironmentService → ToolingExecutor → SDK → Engine
///
/// **Responsibilities:**
/// - Route operations through command spine
/// - Emit environment_changed events through EventBus
/// - Under 150 lines
pub struct EnvironmentService {
    event_bus: Option<Arc<dyn EventBusInterface>>,
}

impl EnvironmentService {
    pub fn new() -> Self {
        Self { event_bus: None }
    }

    /// Create service with EventBus for event-based communication
    ///
    /// **Requirement 29.2, 29.3**
    pub fn new_with_event_bus<E>(event_bus: Arc<E>) -> Self
    where
        E: EventBusInterface + 'static,
    {
        Self {
            event_bus: Some(event_bus),
        }
    }

    /// Initialize service
    ///
    /// **Requirement 5.3**: Services provide lifecycle methods
    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialization logic here (if needed)
        Ok(())
    }

    /// Shutdown service
    ///
    /// **Requirement 5.3**: Services provide lifecycle methods
    pub fn shutdown(&mut self) -> Result<(), String> {
        // Cleanup logic here (if needed)
        Ok(())
    }

    /// Configure sky parameters
    ///
    /// **Preconditions:**
    /// - World must be open
    ///
    /// **Events Emitted:**
    /// - environment_changed(Sky) through EventBus
    ///
    /// **Canonical Route:**
    /// route.environment.configure_sky.v1
    pub fn configure_sky(
        &self,
        config: SkyConfiguration,
    ) -> Result<EnvironmentChangedEvent, String> {
        // Validate preconditions
        if config.sun_intensity < 0.0 || config.sun_intensity > 10.0 {
            return Err("Sun intensity must be in range [0, 10]".to_string());
        }

        // The launch contour resolves the promoted environment command before
        // this service boundary. The service still validates and emits changes.

        let event = EnvironmentChangedEvent {
            change_type: EnvironmentChangeType::Sky,
        };

        // Emit event through EventBus
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::EnvironmentChanged {
                    scope: "environment".to_string(),
                    change_type: "sky_configured".to_string(),
                },
                "EnvironmentService",
            );
        }

        Ok(event)
    }

    /// Configure weather parameters
    ///
    /// **Preconditions:**
    /// - World must be open
    ///
    /// **Events Emitted:**
    /// - environment_changed(Weather) through EventBus
    ///
    /// **Canonical Route:**
    /// route.environment.configure_weather.v1
    pub fn configure_weather(
        &self,
        config: WeatherConfiguration,
    ) -> Result<EnvironmentChangedEvent, String> {
        // Validate preconditions
        if config.precipitation < 0.0 || config.precipitation > 1.0 {
            return Err("Precipitation must be in range [0, 1]".to_string());
        }

        if config.wind_speed < 0.0 {
            return Err("Wind speed must be non-negative".to_string());
        }

        // The launch contour resolves the promoted environment command before
        // this service boundary. The service still validates and emits changes.

        let event = EnvironmentChangedEvent {
            change_type: EnvironmentChangeType::Weather,
        };

        // Emit event through EventBus
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::EnvironmentChanged {
                    scope: "environment".to_string(),
                    change_type: "weather_configured".to_string(),
                },
                "EnvironmentService",
            );
        }

        Ok(event)
    }

    /// Set time of day
    ///
    /// **Preconditions:**
    /// - World must be open
    /// - Time must be in valid range [0, 24)
    ///
    /// **Events Emitted:**
    /// - environment_changed(Time) through EventBus
    ///
    /// **Canonical Route:**
    /// route.environment.set_time.v1
    pub fn set_time(&self, hours: f32) -> Result<EnvironmentChangedEvent, String> {
        // Validate preconditions
        if !(0.0..24.0).contains(&hours) {
            return Err("Time must be in range [0, 24)".to_string());
        }

        // The launch contour resolves the promoted environment command before
        // this service boundary. The service still validates and emits changes.

        let event = EnvironmentChangedEvent {
            change_type: EnvironmentChangeType::Time,
        };

        // Emit event through EventBus
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::EnvironmentChanged {
                    scope: "environment".to_string(),
                    change_type: "time_set".to_string(),
                },
                "EnvironmentService",
            );
        }

        Ok(event)
    }
}

impl Default for EnvironmentService {
    fn default() -> Self {
        Self::new()
    }
}
