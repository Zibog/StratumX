// AppRuntime stub - headless update loop
use crate::editor_host::EditorHost;

pub struct AppRuntime {
    host: EditorHost,
    frame_count: u64,
}

impl AppRuntime {
    pub fn new(host: EditorHost) -> Self {
        Self {
            host,
            frame_count: 0,
        }
    }
    pub fn run_headless(&mut self, max_frames: Option<u64>) -> Result<(), String> {
        let limit = max_frames.unwrap_or(3);
        while self.frame_count < limit {
            let host = &mut self.host;
            host.update(0.016)?;
            self.frame_count += 1;
        }
        Ok(())
    }
}
