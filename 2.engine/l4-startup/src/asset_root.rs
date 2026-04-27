// Asset Root Resolution
// Resolves asset paths from multiple sources

use std::env;
use std::path::{Path, PathBuf};

/// Asset root resolver
/// Tries multiple strategies to find asset root
pub struct AssetRootResolver {
    resolved_root: Option<PathBuf>,
}

impl AssetRootResolver {
    pub fn new() -> Self {
        Self {
            resolved_root: None,
        }
    }

    /// Resolve asset root using multiple strategies
    pub fn resolve(&mut self) -> Result<PathBuf, String> {
        // Strategy 1: STRATUMX_ASSET_ROOT env var
        if let Ok(env_root) = env::var("STRATUMX_ASSET_ROOT") {
            let path = PathBuf::from(env_root);
            if path.exists() {
                self.resolved_root = Some(path.clone());
                return Ok(path);
            }
        }

        // Strategy 2: Workspace root (look for Cargo.toml)
        if let Ok(current_dir) = env::current_dir() {
            // Try current directory
            if Self::is_workspace_root(&current_dir) {
                self.resolved_root = Some(current_dir.clone());
                return Ok(current_dir);
            }

            // Try parent directories (up to 5 levels)
            let mut check_dir = current_dir.clone();
            for _ in 0..5 {
                if let Some(parent) = check_dir.parent() {
                    if Self::is_workspace_root(parent) {
                        self.resolved_root = Some(parent.to_path_buf());
                        return Ok(parent.to_path_buf());
                    }
                    check_dir = parent.to_path_buf();
                } else {
                    break;
                }
            }
        }

        // Strategy 3: Exe directory
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Try exe directory
                if Self::is_workspace_root(exe_dir) {
                    self.resolved_root = Some(exe_dir.to_path_buf());
                    return Ok(exe_dir.to_path_buf());
                }

                // Try parent of exe directory
                if let Some(parent) = exe_dir.parent() {
                    if Self::is_workspace_root(parent) {
                        self.resolved_root = Some(parent.to_path_buf());
                        return Ok(parent.to_path_buf());
                    }
                }
            }
        }

        Err("Failed to resolve asset root - no valid workspace found".to_string())
    }

    /// Check if directory is workspace root
    fn is_workspace_root(path: &Path) -> bool {
        // Check for Cargo.toml (workspace marker)
        let cargo_toml = path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return false;
        }

        // Check for 9.assets directory
        let assets_dir = path.join("9.assets");
        assets_dir.exists() && assets_dir.is_dir()
    }

    /// Get resolved root (if already resolved)
    pub fn get_resolved_root(&self) -> Option<&PathBuf> {
        self.resolved_root.as_ref()
    }

    /// Resolve asset path relative to root
    pub fn resolve_asset_path(&self, relative_path: &str) -> Result<PathBuf, String> {
        if let Some(root) = &self.resolved_root {
            let full_path = root.join(relative_path);
            if full_path.exists() {
                Ok(full_path)
            } else {
                Err(format!("Asset not found: {}", full_path.display()))
            }
        } else {
            Err("Asset root not resolved yet".to_string())
        }
    }

    /// Check if asset exists
    pub fn asset_exists(&self, relative_path: &str) -> bool {
        if let Some(root) = &self.resolved_root {
            let full_path = root.join(relative_path);
            full_path.exists()
        } else {
            false
        }
    }
}

impl Default for AssetRootResolver {
    fn default() -> Self {
        Self::new()
    }
}
