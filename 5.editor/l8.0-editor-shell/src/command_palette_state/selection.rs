//! Command palette selection navigation.

use super::query::CommandPaletteState;

impl CommandPaletteState {
    pub fn move_up(&mut self, item_count: usize) {
        if item_count == 0 {
            self.selected_index = 0;
        } else if self.selected_index == 0 {
            self.selected_index = item_count - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self, item_count: usize) {
        if item_count == 0 {
            self.selected_index = 0;
        } else {
            self.selected_index = (self.selected_index + 1) % item_count;
        }
    }
}
