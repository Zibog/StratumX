use engine_core::{StableDigest64, StableDigestBuilder};
use serde::{Deserialize, Serialize};

/// Content digest for deterministic identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ContentDigest(pub StableDigest64);

impl ContentDigest {
    pub const fn stable(self) -> StableDigest64 {
        self.0
    }

    pub const fn as_u64(self) -> u64 {
        let StableDigest64(value) = self.0;
        value
    }
}

/// Chunk digest for deterministic chunk identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChunkDigest(pub StableDigest64);

impl ChunkDigest {
    pub const fn stable(self) -> StableDigest64 {
        self.0
    }

    pub const fn as_u64(self) -> u64 {
        let StableDigest64(value) = self.0;
        value
    }
}

pub(crate) fn compute_content_digest(bytes: &[u8]) -> ContentDigest {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.content.bytes")
        .write_bytes(bytes);
    ContentDigest(digest.finish())
}

pub(crate) fn compute_chunk_digest(chunk_bytes: &[u8]) -> ChunkDigest {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.content.chunk")
        .write_bytes(chunk_bytes);
    ChunkDigest(digest.finish())
}
