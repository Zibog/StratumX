use engine_core::{StableDigest64, StableDigestBuilder};
use serde::{Deserialize, Serialize};

use crate::digest::{compute_chunk_digest, compute_content_digest};
use crate::{
    ChunkDigest, ContentDigest, ContentFailure, ContentFailureReason, ContentManifest,
    ContentPipelineResult,
};

/// Unique identifier for a runtime pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimePackId(pub u64);

/// Runtime pack manifest with deterministic digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimePackManifest {
    pub pack_id: RuntimePackId,
    pub content_digest: ContentDigest,
    pub chunk_digests: Vec<ChunkDigest>,
    pub dependency_pack_ids: Vec<RuntimePackId>,
    pub dependency_count: usize,
    pub deterministic_digest: StableDigest64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimePackProduct {
    pub pack_ids: Vec<u64>,
    pub deterministic_digest: StableDigest64,
}

pub(crate) fn build_runtime_pack_product(manifest: &ContentManifest) -> RuntimePackProduct {
    let pack_ids: Vec<u64> = manifest.packs.iter().map(|pack| pack.pack_id).collect();
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.content.runtime_pack_product")
        .write_u64(pack_ids.len() as u64)
        .write_u64(manifest.locators.len() as u64);
    for pack_id in &pack_ids {
        digest.write_u64(*pack_id);
    }
    for locator in &manifest.locators {
        digest.write_bytes(locator.uri.as_bytes());
    }
    RuntimePackProduct {
        pack_ids,
        deterministic_digest: digest.finish(),
    }
}

pub(crate) fn build_runtime_pack_manifest(
    pack_id: RuntimePackId,
    content_bytes: &[u8],
    chunk_bytes_list: &[Vec<u8>],
    dependency_pack_ids: &[RuntimePackId],
) -> ContentPipelineResult<RuntimePackManifest> {
    if content_bytes.is_empty() {
        return Err(ContentFailure::new(
            ContentFailureReason::EmptyContent,
            "cannot build manifest from empty content",
        ));
    }

    let content_digest = compute_content_digest(content_bytes);
    let mut chunk_digests = Vec::new();

    for chunk_bytes in chunk_bytes_list {
        if chunk_bytes.is_empty() {
            return Err(ContentFailure::for_reason(
                ContentFailureReason::ChunkSizeZero,
            ));
        }
        chunk_digests.push(compute_chunk_digest(chunk_bytes));
    }

    chunk_digests.sort_by_key(|digest| digest.0);
    let dependency_pack_ids = normalize_dependencies(pack_id, dependency_pack_ids)?;
    let deterministic_digest = runtime_pack_digest(
        pack_id,
        content_digest,
        &chunk_digests,
        &dependency_pack_ids,
    );

    Ok(RuntimePackManifest {
        pack_id,
        content_digest,
        chunk_digests,
        dependency_pack_ids: dependency_pack_ids.clone(),
        dependency_count: dependency_pack_ids.len(),
        deterministic_digest,
    })
}

fn runtime_pack_digest(
    pack_id: RuntimePackId,
    content_digest: ContentDigest,
    chunk_digests: &[ChunkDigest],
    dependency_pack_ids: &[RuntimePackId],
) -> StableDigest64 {
    let mut digest = StableDigestBuilder::new();
    digest
        .write_bytes(b"engine.content.runtime_pack")
        .write_u64(pack_id.0)
        .write_u64(content_digest.as_u64())
        .write_u64(chunk_digests.len() as u64)
        .write_u64(dependency_pack_ids.len() as u64);
    for chunk_digest in chunk_digests {
        digest.write_u64(chunk_digest.as_u64());
    }
    for dependency_pack_id in dependency_pack_ids {
        digest.write_u64(dependency_pack_id.0);
    }
    digest.finish()
}

fn normalize_dependencies(
    pack_id: RuntimePackId,
    dependency_pack_ids: &[RuntimePackId],
) -> ContentPipelineResult<Vec<RuntimePackId>> {
    let mut dependency_pack_ids = dependency_pack_ids.to_vec();
    dependency_pack_ids.sort_by_key(|dependency| dependency.0);

    let mut deduped = Vec::with_capacity(dependency_pack_ids.len());
    for dependency in dependency_pack_ids {
        if dependency == pack_id {
            return Err(ContentFailure::for_reason(
                ContentFailureReason::DependencyCycle,
            ));
        }
        if deduped.last() == Some(&dependency) {
            return Err(ContentFailure::for_reason(
                ContentFailureReason::MissingDependency,
            ));
        }
        deduped.push(dependency);
    }

    Ok(deduped)
}
