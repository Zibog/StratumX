use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentConfig {
    pub max_packs: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentDescriptor {
    pub content_id: u64,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentLocator {
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentPack {
    pub pack_id: u64,
    pub chunk_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentManifest {
    pub packs: Vec<ContentPack>,
    pub locators: Vec<ContentLocator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentRequest {
    pub descriptor: ContentDescriptor,
    pub bytes: Vec<u8>,
    pub locator: ContentLocator,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentResult {
    pub pack: ContentPack,
    pub manifest: ContentManifest,
}
