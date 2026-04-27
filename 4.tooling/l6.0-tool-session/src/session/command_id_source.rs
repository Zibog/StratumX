//! Monotonically increasing command ID source.

pub struct CommandIdSource {
    next: u64,
}

impl Default for CommandIdSource {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandIdSource {
    pub fn new() -> Self {
        Self { next: 1 }
    }

    pub fn next_id(&mut self) -> u64 {
        let id = self.next;
        self.next += 1;
        id
    }
}
