// Asset Command Handlers

use super::session::EditorAuthoringSession;
use link_egress_observations::*;
use link_ingress_packets::AssetCommand;

pub fn handle(
    session: &mut EditorAuthoringSession,
    cmd: AssetCommand,
) -> Result<EditorAuthoringObservation, String> {
    match cmd {
        AssetCommand::ImportStaticMesh { path, label } => {
            let asset_id = session.next_asset_id;
            session.next_asset_id += 1;

            let asset = AssetDto {
                asset_id,
                label,
                asset_type: "StaticMesh".to_string(),
                path,
            };

            session.assets.insert(asset_id, asset.clone());
            Ok(EditorAuthoringObservation::AssetImported { asset })
        }
        AssetCommand::ImportTexture { path, label } => {
            let asset_id = session.next_asset_id;
            session.next_asset_id += 1;

            let asset = AssetDto {
                asset_id,
                label,
                asset_type: "Texture".to_string(),
                path,
            };

            session.assets.insert(asset_id, asset.clone());
            Ok(EditorAuthoringObservation::AssetImported { asset })
        }
        AssetCommand::List => {
            let assets: Vec<AssetDto> = session.assets.values().cloned().collect();
            Ok(EditorAuthoringObservation::AssetList { assets })
        }
        AssetCommand::GetDetails { asset_id } => {
            if let Some(asset) = session.assets.get(&asset_id) {
                Ok(EditorAuthoringObservation::AssetDetails {
                    asset: asset.clone(),
                })
            } else {
                Err(format!("Asset {} not found", asset_id))
            }
        }
    }
}
