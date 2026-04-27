use crate::owners::world_owner::WorldOwner;
use crate::queries::ReadModel;
use std::path::PathBuf;
use uuid::Uuid;

/// World identity view
///
/// Read-only view of world identity information.
#[derive(Debug, Clone)]
pub struct WorldIdentityView {
    pub world_id: Uuid,
    pub world_name: String,
    pub world_path: PathBuf,
    pub snapshot_ref: String,
}

impl ReadModel<WorldOwner, WorldIdentityView> for WorldIdentityView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            world_id: owner.get_world_identity().world_id,
            world_name: owner.get_world_identity().world_name.clone(),
            world_path: owner.get_world_identity().world_path.clone(),
            snapshot_ref: owner.get_snapshot_ref().to_string(),
        }
    }
}
