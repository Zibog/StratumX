//! Monotonically increasing request ID source for packet sessions.

pub struct RequestIdSource {
    next: u64,
}

impl Default for RequestIdSource {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestIdSource {
    pub fn new() -> Self {
        Self { next: 1 }
    }

    pub fn next_id(&mut self) -> u64 {
        let id = self.next;
        self.next += 1;
        id
    }
}
