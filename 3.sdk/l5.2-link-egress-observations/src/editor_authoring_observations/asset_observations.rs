use super::world::AssetDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssetObservation {
    AssetImported {
        asset: AssetDto,
    },
    AssetList {
        assets: Vec<AssetDto>,
    },
    AssetDetails {
        asset: AssetDto,
    },
}
