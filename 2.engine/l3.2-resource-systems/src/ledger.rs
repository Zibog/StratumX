use std::collections::BTreeMap;

use crate::{ContentDigest, ContentFailure, ContentFailureReason, ContentPipelineResult};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContentLedgerEntry {
    locator_uri: String,
    content_digest: ContentDigest,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ContentLedger {
    entries_by_pack_id: BTreeMap<u64, ContentLedgerEntry>,
    pack_id_by_locator: BTreeMap<String, u64>,
    pack_id_by_digest: BTreeMap<ContentDigest, u64>,
}

impl ContentLedger {
    pub(crate) fn record_ingest(
        &mut self,
        pack_id: u64,
        locator_uri: &str,
        content_digest: ContentDigest,
    ) -> ContentPipelineResult<()> {
        if let Some(existing) = self.entries_by_pack_id.get(&pack_id) {
            if existing.content_digest == content_digest && existing.locator_uri == locator_uri {
                return Ok(());
            }
            return Err(ContentFailure::for_reason(
                ContentFailureReason::ContentIdConflict,
            ));
        }

        if let Some(existing_pack_id) = self.pack_id_by_locator.get(locator_uri) {
            if *existing_pack_id != pack_id {
                return Err(ContentFailure::for_reason(
                    ContentFailureReason::LocatorConflict,
                ));
            }
        }

        if let Some(existing_pack_id) = self.pack_id_by_digest.get(&content_digest) {
            if *existing_pack_id != pack_id {
                return Err(ContentFailure::for_reason(
                    ContentFailureReason::DuplicateContentRejected,
                ));
            }
        }

        self.entries_by_pack_id.insert(
            pack_id,
            ContentLedgerEntry {
                locator_uri: locator_uri.to_string(),
                content_digest,
            },
        );
        self.pack_id_by_locator
            .insert(locator_uri.to_string(), pack_id);
        self.pack_id_by_digest.insert(content_digest, pack_id);
        Ok(())
    }
}
