//! ID and clock fixture for testing

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Generates a unique test ID
pub fn next_test_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Test clock that can be controlled
pub struct TestClock {
    current_time: AtomicU64,
}

impl TestClock {
    pub fn new() -> Self {
        Self {
            current_time: AtomicU64::new(0),
        }
    }

    pub fn now(&self) -> u64 {
        self.current_time.load(Ordering::SeqCst)
    }

    pub fn advance(&self, delta: u64) {
        self.current_time.fetch_add(delta, Ordering::SeqCst);
    }

    pub fn set(&self, time: u64) {
        self.current_time.store(time, Ordering::SeqCst);
    }
}

impl Default for TestClock {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a test clock
pub fn create_test_clock() -> TestClock {
    TestClock::new()
}
