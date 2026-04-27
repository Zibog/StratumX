use engine_core::{EngineCoreError, EngineCoreResult};
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
pub struct RuntimePackProduct {
    pub pack_ids: Vec<u64>,
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

#[derive(Debug, Clone)]
pub struct ContentPipeline {
    config: ContentConfig,
}

impl ContentPipeline {
    pub fn new(config: ContentConfig) -> Self {
        Self { config }
    }
    pub fn ingest(&self, request: ContentRequest) -> EngineCoreResult<ContentResult> {
        if request.bytes.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "content ingest requires non-empty bytes",
            ));
        }
        let pack = ContentPack {
            pack_id: request.descriptor.content_id,
            chunk_count: 1,
        };
        let manifest = ContentManifest {
            packs: vec![pack.clone()],
            locators: vec![request.locator],
        };
        if manifest.packs.len() > self.config.max_packs {
            return Err(EngineCoreError::InvalidDescriptor(
                "content manifest exceeds configured pack ceiling",
            ));
        }
        Ok(ContentResult { pack, manifest })
    }
    pub fn build_runtime_pack_product(&self, manifest: &ContentManifest) -> RuntimePackProduct {
        RuntimePackProduct {
            pack_ids: manifest.packs.iter().map(|pack| pack.pack_id).collect(),
        }
    }
}
