use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildTarget {
    Windows,
    Linux,
    MacOS,
    WebGL,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildConfiguration {
    Debug,
    Release,
    Shipping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub target: BuildTarget,
    pub configuration: BuildConfiguration,
    pub output_path: String,
    pub compression_enabled: bool,
}

impl BuildConfig {
    pub fn new(target: BuildTarget, configuration: BuildConfiguration) -> Self {
        Self {
            target,
            configuration,
            output_path: String::from("./build"),
            compression_enabled: true,
        }
    }
}