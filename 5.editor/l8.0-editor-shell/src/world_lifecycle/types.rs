// World Lifecycle Types

/// World role definitions (canon: 45_EDITOR_BOOT_AND_LEVEL_SPACE_CANON.md)
#[derive(Debug, Clone, PartialEq)]
pub enum WorldRole {
    /// Startup world: the world the editor opens by policy when no stronger restore source exists
    Startup,
    /// Reference world: a world used to validate the first product-result loop
    Reference,
    /// Content world: user-created world for game content
    Content,
}

/// Opened world result - returned after successful world load
pub struct OpenedWorld {
    pub world_ref: editor_dto_law::StableWorldId,
    pub bind_state: editor_dto_law::WorldBindState,
    pub world_state: engine_world::WorldState,
    pub label: String,
}
