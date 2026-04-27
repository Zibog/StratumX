//! Ingress Terrain: проверка импорта террейна

use link_ingress_packets::{SurfacePaintInput, TerrainCommand, TerrainPatchInput};

#[test]
fn import_terrain_heightmap() {
    let command = TerrainCommand::CreatePatch {
        input: TerrainPatchInput {
            position: [10.0, 2.0, -4.0],
            size: [64.0, 32.0],
            label: "ridge".to_string(),
        },
    };
    let decoded: TerrainCommand =
        serde_json::from_str(&serde_json::to_string(&command).unwrap()).unwrap();

    match decoded {
        TerrainCommand::CreatePatch { input } => {
            assert_eq!(input.position, [10.0, 2.0, -4.0]);
            assert_eq!(input.size, [64.0, 32.0]);
            assert_eq!(input.label, "ridge");
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn import_terrain_materials() {
    let command = TerrainCommand::PaintSurfaceStack {
        target_entity_id: 42,
        paint: SurfacePaintInput {
            center: [1.0, 0.0, 2.0],
            radius: 3.5,
            stack_id: 9,
        },
    };
    let decoded: TerrainCommand =
        serde_json::from_value(serde_json::to_value(&command).unwrap()).unwrap();

    match decoded {
        TerrainCommand::PaintSurfaceStack {
            target_entity_id,
            paint,
        } => {
            assert_eq!(target_entity_id, 42);
            assert_eq!(paint.radius, 3.5);
            assert_eq!(paint.stack_id, 9);
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn import_terrain_metadata() {
    let decoded: TerrainCommand =
        serde_json::from_str(r#"{"GetPatchDetails":{"patch_id":7}}"#).unwrap();

    match decoded {
        TerrainCommand::GetPatchDetails { patch_id } => assert_eq!(patch_id, 7),
        other => panic!("unexpected command: {other:?}"),
    }
}
