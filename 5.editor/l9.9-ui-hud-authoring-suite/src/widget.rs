//! UI Widget system
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WidgetId(pub Uuid);

impl WidgetId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    Button, Label, TextInput, Image, Panel, Slider, Checkbox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    pub id: WidgetId,
    pub widget_type: WidgetType,
    pub name: String,
    pub visible: bool,
    pub enabled: bool,
    pub children: Vec<WidgetId>,
}

impl Widget {
    pub fn new(name: String, widget_type: WidgetType) -> Self {
        Self {
            id: WidgetId::new(),
            widget_type,
            name,
            visible: true,
            enabled: true,
            children: Vec::new(),
        }
    }
}