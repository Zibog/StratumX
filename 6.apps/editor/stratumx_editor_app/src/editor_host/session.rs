use engine_world::WorldState;

#[derive(Debug)]
pub struct EditorSession {
    pub world: WorldState,
    pub world_label: String,
    pub world_path: Option<std::path::PathBuf>,
}
