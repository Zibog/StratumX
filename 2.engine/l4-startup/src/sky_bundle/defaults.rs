// Sky bundle status types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetStatus {
    Found,
    Missing,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BundleCompleteness {
    Complete,
    Partial,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkyBundleStatus {
    pub manifest_loaded: bool,
    pub bundle_id: Option<String>,
    pub completeness: BundleCompleteness,
    pub stars_status: AssetStatus,
    pub moon_albedo_status: AssetStatus,
    pub moon_height_status: AssetStatus,
    pub moon_normal_status: AssetStatus,
    pub sun_disk_status: AssetStatus,
    pub blue_noise_status: AssetStatus,
    pub noise_source_status: AssetStatus,
    pub noise_source_count: usize,
}

impl SkyBundleStatus {
    pub fn not_loaded() -> Self {
        Self {
            manifest_loaded: false,
            bundle_id: None,
            completeness: BundleCompleteness::Invalid,
            stars_status: AssetStatus::Missing,
            moon_albedo_status: AssetStatus::Missing,
            moon_height_status: AssetStatus::NotRequired,
            moon_normal_status: AssetStatus::NotRequired,
            sun_disk_status: AssetStatus::NotRequired,
            blue_noise_status: AssetStatus::Missing,
            noise_source_status: AssetStatus::Missing,
            noise_source_count: 0,
        }
    }

    pub fn is_production_ready(&self) -> bool {
        self.manifest_loaded && self.completeness == BundleCompleteness::Complete
    }

    pub fn is_functional(&self) -> bool {
        self.manifest_loaded && self.completeness != BundleCompleteness::Invalid
    }
}
