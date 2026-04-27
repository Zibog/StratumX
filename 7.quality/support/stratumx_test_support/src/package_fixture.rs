//! Package fixture for testing

use editor_dto_law::{SourceLineage, WorldPackageManifest, WorldRole};
use uuid::Uuid;

/// Creates a minimal world package manifest
pub fn create_test_package() -> WorldPackageManifest {
    WorldPackageManifest {
        world_id: Uuid::from_u128(0x0000_0000_0000_0001),
        world_label: "Test World".to_string(),
        world_role: WorldRole::Demo,
        version: "1.0.0".to_string(),
        terrain_root_ref: None,
        environment_root_ref: None,
        streaming_profile_ref: None,
        source_lineage: SourceLineage {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            created_by: "test".to_string(),
            import_source: None,
            last_modified: "2024-01-01T00:00:00Z".to_string(),
        },
    }
}
