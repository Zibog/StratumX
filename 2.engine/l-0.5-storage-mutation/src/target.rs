use engine_core::{ComponentTypeId, StableDigest64, StableDigestBuilder};
use smallvec::SmallVec;
use std::collections::BTreeMap;

use crate::{ApplyContext, CloneProjectionApplyTransaction, MutationApplyTransaction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedJournal {
    pub journal_digest: StableDigest64,
    pub before_digest: StableDigest64,
    pub after_digest: StableDigest64,
    pub contains_non_idempotent_write: bool,
}

pub trait MutationApplyTarget {
    type Transaction<'a>: MutationApplyTransaction
    where
        Self: 'a;

    fn component_bytes(&self, component: ComponentTypeId) -> Option<&[u8]>;
    fn has_component(&self, component: ComponentTypeId) -> bool;
    fn stable_state_digest(&self) -> StableDigest64;
    fn applied_journal(&self, journal_digest: StableDigest64) -> Option<AppliedJournal>;
    fn begin_apply<'a>(&'a mut self, context: ApplyContext) -> Self::Transaction<'a>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InMemoryMutationApplyTarget {
    pub(crate) components: BTreeMap<ComponentTypeId, SmallVec<[u8; 32]>>,
    pub(crate) applied_journals: BTreeMap<StableDigest64, AppliedJournal>,
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

    pub fn stable_state_digest(&self) -> StableDigest64 {
        <Self as MutationApplyTarget>::stable_state_digest(self)
    }

    pub fn deterministic_digest(&self) -> StableDigest64 {
        self.stable_state_digest()
    }
}

impl MutationApplyTarget for InMemoryMutationApplyTarget {
    type Transaction<'a>
        = CloneProjectionApplyTransaction<'a>
    where
        Self: 'a;

    fn component_bytes(&self, component: ComponentTypeId) -> Option<&[u8]> {
        self.components.get(&component).map(SmallVec::as_slice)
    }

    fn has_component(&self, component: ComponentTypeId) -> bool {
        self.components.contains_key(&component)
    }

    fn stable_state_digest(&self) -> StableDigest64 {
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

    fn begin_apply<'a>(&'a mut self, context: ApplyContext) -> Self::Transaction<'a> {
        CloneProjectionApplyTransaction::new(self, context)
    }
}

pub type LegacyMutationApplyTarget = InMemoryMutationApplyTarget;
