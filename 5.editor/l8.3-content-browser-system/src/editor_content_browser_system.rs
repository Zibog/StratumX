//! FUTURE_STUB.
//!
//! This crate is present as a canonical future surface.
//! It is not part of the active product spine yet.
//! It must not be counted as product-complete.

//! Editor Content Browser System
//!
//! Role: Asset browsing and file system navigation.
//! Owns: Content browser tree, asset filters, thumbnail cache.

pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Model,
    Texture,
    Material,
    Audio,
    Script,
    Config,
    Folder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetEntry {
    pub path: String,
    pub name: String,
    pub asset_type: AssetType,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentBrowserSystem {
    pub current_path: String,
    pub entries: Vec<AssetEntry>,
    pub selected_ids: Vec<String>,
    pub filter_text: String,
}

impl ContentBrowserSystem {
    pub fn new() -> Self {
        Self {
            current_path: "/".to_string(),
            entries: Vec::new(),
            selected_ids: Vec::new(),
            filter_text: String::new(),
        }
    }
}
