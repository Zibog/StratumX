use super::tracker::CommandEnvelope;

pub(crate) fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

impl CommandEnvelope {
    /// Get the duration since command was created (in milliseconds).
    pub fn age_ms(&self) -> u64 {
        now_ms().saturating_sub(self.origin_timestamp)
    }
}
