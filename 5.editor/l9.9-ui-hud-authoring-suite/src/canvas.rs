//! UI Canvas management
use crate::widget::{Widget, WidgetId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanvasId(pub Uuid);

impl CanvasId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub id: CanvasId,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub widgets: HashMap<WidgetId, Widget>,
    pub root_widgets: Vec<WidgetId>,
}

impl Canvas {
    pub fn new(name: String, width: f32, height: f32) -> Self {
        Self {
            id: CanvasId::new(),
            name,
            width,
            height,
            widgets: HashMap::new(),
            root_widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        let id = widget.id;
        self.widgets.insert(id, widget);
        self.root_widgets.push(id);
    }
}