// Waiver Registry for Hygiene Rules.

mod collections;
mod file_io;
mod matching;
mod validation;

use crate::models::{HygieneRule, WaiverError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use collections::WaiverEntry;

/// Main waiver registry structure that holds all waiver entries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WaiverRegistry {
    #[serde(default)]
    pub line_limit_waivers: Vec<WaiverEntry>,
    #[serde(default)]
    pub allow_attr_waivers: Vec<WaiverEntry>,
    #[serde(default)]
    pub test_in_src_waivers: Vec<WaiverEntry>,
    #[serde(default)]
    pub execute_bridge_waivers: Vec<WaiverEntry>,
}

impl WaiverRegistry {
    /// Create a new empty waiver registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load waiver registry from a TOML file.
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        file_io::load_from_file(path)
    }

    /// Check if a specific file and rule combination is waived.
    pub fn is_waived(&self, rule: &HygieneRule, file: &Path) -> bool {
        matching::is_waived(self, rule, file)
    }

    /// Validate all waiver entries to ensure referenced files exist.
    pub fn validate_waivers(&self, repo_root: &Path) -> Vec<WaiverError> {
        validation::validate_waivers(self, repo_root)
    }

    /// Get all waived paths for a specific rule type.
    pub fn get_waivers_for_rule(&self, rule: &HygieneRule) -> HashMap<PathBuf, String> {
        matching::get_waivers_for_rule(self, rule)
    }
}

#[cfg(test)]
mod tests;
