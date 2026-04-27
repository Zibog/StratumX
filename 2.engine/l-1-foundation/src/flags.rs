#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureProfile {
    Minimal,
    Headless,
    Realtime,
}

pub const ACTIVE_PROFILE: FeatureProfile = if cfg!(feature = "realtime") {
    FeatureProfile::Realtime
} else if cfg!(feature = "headless") {
    FeatureProfile::Headless
} else {
    FeatureProfile::Minimal
};
