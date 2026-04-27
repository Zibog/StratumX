// Streaming seed - streaming и residency defaults

use engine_world::StreamingManager;

pub fn create_streaming_manager() -> StreamingManager {
    StreamingManager::new(512) // 512MB budget
}
