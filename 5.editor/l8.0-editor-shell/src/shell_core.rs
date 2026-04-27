//! Shell Core - Thin shell host
//!
//! Per canon 02_STACK_MAP: L8.0 = shell host only, no domain logic.
//! Domain logic lives in L8.1+ systems.

pub struct EditorShell {
    pub project_id: Option<String>,
    pub active_panels: Vec<String>,
}

impl EditorShell {
    pub fn new() -> Self {
        Self {
            project_id: None,
            active_panels: Vec::new(),
        }
    }
}
