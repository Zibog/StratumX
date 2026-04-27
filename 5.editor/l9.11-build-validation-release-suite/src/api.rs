use crate::build_config::*;
use crate::validation::*;
use crate::packaging::*;

pub struct BuildReleaseService {
    current_config: Option<BuildConfig>,
}

impl BuildReleaseService {
    pub fn new() -> Self {
        Self {
            current_config: None,
        }
    }

    pub fn set_config(&mut self, config: BuildConfig) {
        self.current_config = Some(config);
    }

    pub fn validate(&self) -> ValidationResult {
        validate_build()
    }

    pub fn create_package(&self, name: String, version: String) -> Package {
        Package::new(name, version)
    }
}

impl Default for BuildReleaseService {
    fn default() -> Self {
        Self::new()
    }
}