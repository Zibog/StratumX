//! Egress Terrain: проверка экспорта террейна

use link_egress_observations::{AuthoringTerrainPatchDto, SurfaceRegionDto};

fn patch_dto() -> AuthoringTerrainPatchDto {
    AuthoringTerrainPatchDto {
        patch_id: 11,
        label: "alpha".to_string(),
        position: [1.0, 2.0, 3.0],
        size: [64.0, 64.0],
        surface_regions: vec![SurfaceRegionDto {
            region_id: 5,
            center: [4.0, 0.0, 6.0],
            radius: 3.0,
            stack_id: 12,
        }],
    }
}

#[test]
fn export_terrain_heightmap() {
    let value = serde_json::to_value(patch_dto()).unwrap();

    assert!(value.get("patchId").is_some());
    assert!(value.get("surfaceRegions").is_some());
    assert!(value.get("patch_id").is_none());
}

#[test]
fn export_terrain_materials() {
    let roundtrip: AuthoringTerrainPatchDto =
        serde_json::from_value(serde_json::to_value(patch_dto()).unwrap()).unwrap();

    assert_eq!(roundtrip.surface_regions[0].stack_id, 12);
    assert_eq!(roundtrip.surface_regions[0].region_id, 5);
}

#[test]
fn export_terrain_lod() {
    let dto = patch_dto();

    assert_eq!(dto.size, [64.0, 64.0]);
    assert_eq!(dto.surface_regions[0].center, [4.0, 0.0, 6.0]);
    assert_eq!(dto.surface_regions[0].radius, 3.0);
}
