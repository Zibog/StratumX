// Sky bundle manifest types and loading

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkyBundleManifest {
    pub bundle_id: String,
    pub stars_exr: String,
    pub moon_albedo: String,
    pub moon_height: Option<String>,
    pub moon_normal: Option<String>,
    pub sun_disk: Option<String>,
    pub blue_noise: String,
    pub noise_source: Vec<String>,
}

pub fn load_sky_bundle_manifest(path: &str) -> Result<SkyBundleManifest, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read manifest: {}", e))?;

    let manifest: SkyBundleManifest =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse manifest: {}", e))?;

    Ok(manifest)
}
