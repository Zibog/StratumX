use engine_core::EngineCoreResult;

use crate::digest::{compute_chunk_digest, compute_content_digest};
use crate::ledger::ContentLedger;
use crate::runtime_pack;
use crate::validation::{validate_descriptor, validate_locator};
use crate::{
    ChunkDigest, ContentConfig, ContentDigest, ContentFailure, ContentFailureReason,
    ContentLocator, ContentManifest, ContentPack, ContentPipelineResult, ContentRequest,
    ContentResult, RuntimePackId, RuntimePackManifest, RuntimePackProduct,
};

#[derive(Debug, Clone)]
pub struct ContentPipeline {
    config: ContentConfig,
    ledger: ContentLedger,
}

impl ContentPipeline {
    pub fn new(config: ContentConfig) -> Self {
        Self {
            config,
            ledger: ContentLedger::default(),
        }
    }

    pub fn try_ingest(&mut self, request: ContentRequest) -> ContentPipelineResult<ContentResult> {
        validate_descriptor(&request.descriptor)?;
        let normalized_locator = validate_locator(&request.locator)?;
        if request.bytes.is_empty() {
            return Err(ContentFailure::for_reason(
                ContentFailureReason::EmptyContent,
            ));
        }

        let content_digest = self.compute_content_digest(&request.bytes);
        self.ledger.record_ingest(
            request.descriptor.content_id,
            &normalized_locator,
            content_digest,
        )?;

        let pack = ContentPack {
            pack_id: request.descriptor.content_id,
            chunk_count: 1,
        };
        let manifest = ContentManifest {
            packs: vec![pack.clone()],
            locators: vec![ContentLocator {
                uri: normalized_locator,
            }],
        };
        if manifest.packs.len() > self.config.max_packs {
            return Err(ContentFailure::for_reason(
                ContentFailureReason::ManifestPackCeiling,
            ));
        }
        Ok(ContentResult { pack, manifest })
    }

    pub fn ingest(&mut self, request: ContentRequest) -> EngineCoreResult<ContentResult> {
        self.try_ingest(request).map_err(Into::into)
    }

    pub fn ingested_pack_count(&self) -> usize {
        self.ledger.ingested_pack_count()
    }

    pub fn has_ingested_pack(&self, pack_id: u64) -> bool {
        self.ledger.has_pack(pack_id)
    }

    pub fn build_runtime_pack_product(&self, manifest: &ContentManifest) -> RuntimePackProduct {
        runtime_pack::build_runtime_pack_product(manifest)
    }

    /// Compute content digest from bytes.
    pub fn compute_content_digest(&self, bytes: &[u8]) -> ContentDigest {
        compute_content_digest(bytes)
    }

    /// Compute chunk digest.
    pub fn compute_chunk_digest(&self, chunk_bytes: &[u8]) -> ChunkDigest {
        compute_chunk_digest(chunk_bytes)
    }

    /// Build runtime pack manifest with deterministic digest.
    pub fn build_runtime_pack_manifest(
        &self,
        pack_id: RuntimePackId,
        content_bytes: &[u8],
        chunk_bytes_list: &[Vec<u8>],
    ) -> EngineCoreResult<RuntimePackManifest> {
        self.try_build_runtime_pack_manifest(pack_id, content_bytes, chunk_bytes_list)
            .map_err(Into::into)
    }

    pub fn try_build_runtime_pack_manifest(
        &self,
        pack_id: RuntimePackId,
        content_bytes: &[u8],
        chunk_bytes_list: &[Vec<u8>],
    ) -> ContentPipelineResult<RuntimePackManifest> {
        self.try_build_runtime_pack_manifest_with_dependencies(
            pack_id,
            content_bytes,
            chunk_bytes_list,
            &[],
        )
    }

    pub fn build_runtime_pack_manifest_with_dependencies(
        &self,
        pack_id: RuntimePackId,
        content_bytes: &[u8],
        chunk_bytes_list: &[Vec<u8>],
        dependency_pack_ids: &[RuntimePackId],
    ) -> EngineCoreResult<RuntimePackManifest> {
        self.try_build_runtime_pack_manifest_with_dependencies(
            pack_id,
            content_bytes,
            chunk_bytes_list,
            dependency_pack_ids,
        )
        .map_err(Into::into)
    }

    pub fn try_build_runtime_pack_manifest_with_dependencies(
        &self,
        pack_id: RuntimePackId,
        content_bytes: &[u8],
        chunk_bytes_list: &[Vec<u8>],
        dependency_pack_ids: &[RuntimePackId],
    ) -> ContentPipelineResult<RuntimePackManifest> {
        runtime_pack::build_runtime_pack_manifest(
            pack_id,
            content_bytes,
            chunk_bytes_list,
            dependency_pack_ids,
        )
    }
}
