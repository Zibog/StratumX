use crate::owners::world_owner::WorldOwner;
use crate::queries::ReadModel;

/// World terrain summary view
///
/// Read-only summary of terrain state.
#[derive(Debug, Clone)]
pub struct WorldTerrainSummaryView {
    pub has_terrain: bool,
    pub heightmap_resolution: Option<(u32, u32)>,
    pub terrain_size: Option<(f32, f32)>,
    pub current_layer: Option<String>,
    pub modification_count: u64,
}

impl ReadModel<WorldOwner, WorldTerrainSummaryView> for WorldTerrainSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        if let Some(terrain) = owner.get_terrain_state() {
            Self {
                has_terrain: true,
                heightmap_resolution: Some(terrain.heightmap_resolution),
                terrain_size: Some(terrain.terrain_size),
                current_layer: Some(terrain.current_layer.clone()),
                modification_count: terrain.modification_count,
            }
        } else {
            Self {
                has_terrain: false,
                heightmap_resolution: None,
                terrain_size: None,
                current_layer: None,
                modification_count: 0,
            }
        }
    }
}
