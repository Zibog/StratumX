//! Public API for UI authoring
use crate::canvas::*;
use crate::widget::*;
use std::collections::HashMap;

pub struct UIAuthoringService {
    canvases: HashMap<CanvasId, Canvas>,
}

impl UIAuthoringService {
    pub fn new() -> Self {
        Self {
            canvases: HashMap::new(),
        }
    }

    pub fn create_canvas(&mut self, name: String, width: f32, height: f32) -> CanvasId {
        let canvas = Canvas::new(name, width, height);
        let id = canvas.id;
        self.canvases.insert(id, canvas);
        id
    }

    pub fn get_canvas(&self, id: CanvasId) -> Option<&Canvas> {
        self.canvases.get(&id)
    }

    pub fn get_canvas_mut(&mut self, id: CanvasId) -> Option<&mut Canvas> {
        self.canvases.get_mut(&id)
    }
}

impl Default for UIAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}