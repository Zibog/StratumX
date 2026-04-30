use engine_core::{ComponentTypeId, StableDigest64, StableDigestBuilder};
use smallvec::SmallVec;
use std::collections::BTreeMap;

use crate::DeferredWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedJournal {
    pub journal_digest: StableDigest64,
    pub before_digest: StableDigest64,
    pub after_digest: StableDigest64,
    pub contains_non_idempotent_write: bool,
}

pub trait MutationApplyTarget {
    fn component_bytes(&self, component: ComponentTypeId) -> Option<&[u8]>;
    fn has_component(&self, component: ComponentTypeId) -> bool;
    fn deterministic_digest(&self) -> StableDigest64;
    fn applied_journal(&self, journal_digest: StableDigest64) -> Option<AppliedJournal>;
    fn apply_structural_removals(&mut self, components: &[ComponentTypeId]);
    fn apply_writes(&mut self, writes: &[DeferredWrite]);
    fn record_journal(&mut self, journal: AppliedJournal);
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InMemoryMutationApplyTarget {
    components: BTreeMap<ComponentTypeId, SmallVec<[u8; 32]>>,
    applied_journals: BTreeMap<StableDigest64, AppliedJournal>,
}

impl InMemoryMutationApplyTarget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_seeded_components<I>(components: I) -> Self
    where
        I: IntoIterator<Item = (ComponentTypeId, SmallVec<[u8; 32]>)>,
    {
        let mut target = Self::new();
        for (component, bytes) in components {
            target.components.insert(component, bytes);
        }
        target
    }

    pub fn component_bytes(&self, component: ComponentTypeId) -> Option<&[u8]> {
        <Self as MutationApplyTarget>::component_bytes(self, component)
    }

    pub fn has_component(&self, component: ComponentTypeId) -> bool {
        <Self as MutationApplyTarget>::has_component(self, component)
    }

    pub fn deterministic_digest(&self) -> StableDigest64 {
        <Self as MutationApplyTarget>::deterministic_digest(self)
    }
}

impl MutationApplyTarget for InMemoryMutationApplyTarget {
    fn component_bytes(&self, component: ComponentTypeId) -> Option<&[u8]> {
        self.components.get(&component).map(SmallVec::as_slice)
    }

    fn has_component(&self, component: ComponentTypeId) -> bool {
        self.components.contains_key(&component)
    }

    fn deterministic_digest(&self) -> StableDigest64 {
        let mut builder = StableDigestBuilder::new();
        builder.write_bytes(b"engine_storage_mutation.target.v1");
        builder.write_u64(self.components.len() as u64);
        for (component, bytes) in &self.components {
            builder.write_u64(component.0);
            builder.write_bytes(bytes);
        }
        builder.finish()
    }

    fn applied_journal(&self, journal_digest: StableDigest64) -> Option<AppliedJournal> {
        self.applied_journals.get(&journal_digest).copied()
    }

    fn apply_structural_removals(&mut self, components: &[ComponentTypeId]) {
        for component in components {
            self.components.remove(component);
        }
    }

    fn apply_writes(&mut self, writes: &[DeferredWrite]) {
        for write in writes {
            self.components.insert(write.component, write.bytes.clone());
        }
    }

    fn record_journal(&mut self, journal: AppliedJournal) {
        self.applied_journals
            .insert(journal.journal_digest, journal);
    }
}

pub type LegacyMutationApplyTarget = InMemoryMutationApplyTarget;
