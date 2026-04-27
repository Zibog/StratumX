pub struct HotReloadManager {
    enabled: bool,
}

impl HotReloadManager {
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}