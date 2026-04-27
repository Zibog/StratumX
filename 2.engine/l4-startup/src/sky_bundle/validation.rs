// Sky bundle validation

use super::defaults::{AssetStatus, BundleCompleteness, SkyBundleStatus};
use super::manifest::{load_sky_bundle_manifest, SkyBundleManifest};
use std::path::Path;

pub fn validate_sky_bundle(manifest: &SkyBundleManifest) -> SkyBundleStatus {
    let stars_status = if Path::new(&manifest.stars_exr).exists() {
        AssetStatus::Found
    } else {
        AssetStatus::Missing
    };

    let moon_albedo_status = if Path::new(&manifest.moon_albedo).exists() {
        AssetStatus::Found
    } else {
        AssetStatus::Missing
    };

    let moon_height_status = if let Some(ref path) = manifest.moon_height {
        if Path::new(path).exists() {
            AssetStatus::Found
        } else {
            AssetStatus::Missing
        }
    } else {
        AssetStatus::NotRequired
    };

    let moon_normal_status = if let Some(ref path) = manifest.moon_normal {
        if Path::new(path).exists() {
            AssetStatus::Found
        } else {
            AssetStatus::Missing
        }
    } else {
        AssetStatus::NotRequired
    };

    let sun_disk_status = if let Some(ref path) = manifest.sun_disk {
        if Path::new(path).exists() {
            AssetStatus::Found
        } else {
            AssetStatus::Missing
        }
    } else {
        AssetStatus::NotRequired
    };

    let blue_noise_status = if Path::new(&manifest.blue_noise).exists() {
        AssetStatus::Found
    } else {
        AssetStatus::Missing
    };

    let mut noise_source_found = 0;
    for noise_path in &manifest.noise_source {
        if Path::new(noise_path).exists() {
            noise_source_found += 1;
        }
    }

    let noise_source_status = if noise_source_found == manifest.noise_source.len() {
        AssetStatus::Found
    } else {
        AssetStatus::Missing
    };

    let completeness = {
        let critical_missing = stars_status == AssetStatus::Missing
            || moon_albedo_status == AssetStatus::Missing
            || blue_noise_status == AssetStatus::Missing
            || noise_source_status == AssetStatus::Missing;

        if critical_missing {
            BundleCompleteness::Invalid
        } else {
            let moon_height_missing = matches!(moon_height_status, AssetStatus::Missing);
            let moon_normal_missing = matches!(moon_normal_status, AssetStatus::Missing);

            if moon_height_missing || moon_normal_missing {
                BundleCompleteness::Partial
            } else {
                BundleCompleteness::Complete
            }
        }
    };

    SkyBundleStatus {
        manifest_loaded: true,
        bundle_id: Some(manifest.bundle_id.clone()),
        completeness,
        stars_status,
        moon_albedo_status,
        moon_height_status,
        moon_normal_status,
        sun_disk_status,
        blue_noise_status,
        noise_source_status,
        noise_source_count: noise_source_found,
    }
}

pub fn get_sky_bundle_status(path: &str) -> SkyBundleStatus {
    match load_sky_bundle_manifest(path) {
        Ok(manifest) => validate_sky_bundle(&manifest),
        Err(_) => SkyBundleStatus::not_loaded(),
    }
}

pub fn get_sky_bundle_status_with_resolver(
    relative_path: &str,
) -> (SkyBundleStatus, Option<String>) {
    use crate::asset_root::AssetRootResolver;

    let mut resolver = AssetRootResolver::new();

    let asset_root = match resolver.resolve() {
        Ok(root) => Some(root.display().to_string()),
        Err(_) => None,
    };

    let bundle_path = if let Some(ref root_str) = asset_root {
        let full_path = std::path::PathBuf::from(root_str).join(relative_path);
        full_path.display().to_string()
    } else {
        relative_path.to_string()
    };

    let status = get_sky_bundle_status(&bundle_path);

    (status, asset_root)
}
