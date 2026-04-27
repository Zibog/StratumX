//! Viewport Service - stub after refactoring

use std::sync::Arc;

pub trait EventBusInterface {}
pub struct ServiceEvent;

#[derive(Debug, Clone, Default)]
pub struct ViewportConfiguration {
    pub id: String,
    pub width: u32,
    pub height: u32,
}

pub struct ViewportService {
    #[allow(dead_code)]
    viewports: Vec<ViewportConfiguration>,
    #[allow(dead_code)]
    active_viewport: usize,
    #[allow(dead_code)]
    event_bus: Option<Arc<dyn EventBusInterface>>,
}

impl ViewportService {
    pub fn new() -> Self {
        Self {
            viewports: vec![],
            active_viewport: 0,
            event_bus: None,
        }
    }
    pub fn initialize(&mut self) -> Result<(), String> { Ok(()) }
    pub fn shutdown(&mut self) -> Result<(), String> { Ok(()) }
}

impl Default for ViewportService {
    fn default() -> Self { Self::new() }
}
