use crate::{ApplyTransactionId, MutationApplyMode, MutationBatchId, MutationConflictPolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyContext {
    pub batch_id: MutationBatchId,
    pub transaction_id: ApplyTransactionId,
    pub mode: MutationApplyMode,
    pub conflict_policy: MutationConflictPolicy,
}
