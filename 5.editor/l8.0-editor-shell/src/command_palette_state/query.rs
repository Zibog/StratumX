//! Command palette query and input handling.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPaletteState {
    pub is_open: bool,
    pub query: String,
    pub selected_index: usize,
}

impl CommandPaletteState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            query: String::new(),
            selected_index: 0,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
        self.query.clear();
        self.selected_index = 0;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn input(&mut self, query: &str) {
        self.query = query.to_string();
        self.selected_index = 0;
    }
}

impl Default for CommandPaletteState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_palette_open_resets_query_and_index() {
        let mut state = CommandPaletteState {
            is_open: false,
            query: "terrain".to_string(),
            selected_index: 3,
        };

        state.open();

        assert!(state.is_open);
        assert!(state.query.is_empty());
        assert_eq!(state.selected_index, 0);
    }
}
