//! Terrain Response: проверка реакции террейна на воздействия

use engine_material::{TerrainBlastResponse, TerrainMaterialType};

#[test]
fn terrain_deformation() {
    let response =
        TerrainBlastResponse::from_explosion([0.0, 0.0, 0.0], 250_000.0, TerrainMaterialType::Dirt);

    assert!(response.crater.radius_m > 0.1);
    assert!(response.crater.depth_m > 0.0);
    assert_eq!(
        response.debris.len(),
        response.crater.debris_count.min(100) as usize
    );
}

#[test]
fn terrain_erosion() {
    let sand =
        TerrainBlastResponse::from_explosion([0.0, 0.0, 0.0], 250_000.0, TerrainMaterialType::Sand);
    let concrete = TerrainBlastResponse::from_explosion(
        [0.0, 0.0, 0.0],
        250_000.0,
        TerrainMaterialType::Concrete,
    );

    assert!(sand.crater.radius_m > concrete.crater.radius_m);
    assert!(sand.crater.depth_m > concrete.crater.depth_m);
}

#[test]
fn terrain_material_change() {
    let low =
        TerrainBlastResponse::from_explosion([1.0, 0.0, 1.0], 50_000.0, TerrainMaterialType::Snow);
    let high =
        TerrainBlastResponse::from_explosion([1.0, 0.0, 1.0], 500_000.0, TerrainMaterialType::Snow);

    assert!(high.crater.ejecta_radius_m > low.crater.ejecta_radius_m);
    assert!(high.crater.ejecta_volume_m3 > low.crater.ejecta_volume_m3);
    assert!(high.dust_cloud_radius_m > low.dust_cloud_radius_m);
}
