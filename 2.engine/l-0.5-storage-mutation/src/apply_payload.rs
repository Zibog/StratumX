use engine_core::{EngineCoreError, EngineCoreResult};
use engine_storage_access::WriteWindow;
use smallvec::SmallVec;

use crate::journal_digest::raw_payload_journal_digest;
use crate::{
    AppliedJournal, ApplyFlags, ApplyPayload, ApplyReceiptContext, ApplyTransactionId,
    AuthoritativeApplyReceipt, ChangeSet, FamilyTag, MutationApplyMode, MutationApplyPlan,
    MutationApplyReport, MutationApplyTarget, MutationBatchId, MutationBuffer,
    MutationConflictPolicy, MutationFailureReason, RegionTag,
};

pub fn queue_deferred_writes(
    window: &WriteWindow,
    buffer: MutationBuffer,
) -> EngineCoreResult<ChangeSet> {
    if !window.descriptor.staged_mutation_handoff {
        return Err(EngineCoreError::InvalidDescriptor(
            "deferred writes require staged handoff window",
        ));
    }
    Ok(buffer.into_change_set(SmallVec::new()))
}

pub fn make_apply_payload(
    family_tag: FamilyTag,
    region_tag: RegionTag,
    batch_order: u64,
    change_set: ChangeSet,
) -> EngineCoreResult<ApplyPayload> {
    if batch_order == 0 {
        return Err(EngineCoreError::InvalidDescriptor(
            "batch order must be non-zero",
        ));
    }
    Ok(ApplyPayload {
        family_tag,
        region_tag,
        batch_order,
        flags: ApplyFlags::SEGMENTED,
        change_set,
    })
}

pub fn authoritative_apply<T>(
    target: &mut T,
    payload: &ApplyPayload,
    batch_id: MutationBatchId,
    transaction_id: ApplyTransactionId,
    mode: MutationApplyMode,
    conflict_policy: MutationConflictPolicy,
) -> AuthoritativeApplyReceipt
where
    T: Clone + MutationApplyTarget,
{
    let before_digest = target.deterministic_digest();
    let fallback_journal_digest = raw_payload_journal_digest(payload);
    let fallback_context = ApplyReceiptContext {
        batch_id,
        transaction_id,
        batch_order: payload.batch_order,
        family_tag: payload.family_tag,
        region_tag: payload.region_tag,
        before_digest,
        journal_digest: fallback_journal_digest,
    };

    let plan = match MutationApplyPlan::from_payload_with_policy(payload, conflict_policy, mode) {
        Ok(plan) => plan,
        Err(error) => {
            return AuthoritativeApplyReceipt::validation_failed(
                fallback_context,
                error.as_failure_reason(),
            );
        }
    };

    let receipt_context = ApplyReceiptContext {
        journal_digest: plan.journal_digest(),
        ..fallback_context
    };

    if let Some(receipt) =
        guarded_replay_receipt(target, &plan, before_digest, mode, receipt_context)
    {
        return receipt;
    }

    if missing_structural_target(target, &plan) {
        return AuthoritativeApplyReceipt::validation_failed(
            receipt_context,
            MutationFailureReason::MissingTarget,
        );
    }

    let mut projected_target = target.clone();
    projected_target.apply_structural_removals(plan.structural_components());
    projected_target.apply_writes(plan.deferred_writes());

    let after_digest = projected_target.deterministic_digest();
    let report = MutationApplyReport::all_applied(plan.operation_count);

    if mode != MutationApplyMode::DryRun {
        *target = projected_target;
        target.record_journal(AppliedJournal {
            journal_digest: plan.journal_digest(),
            before_digest,
            after_digest,
            contains_non_idempotent_write: plan.contains_non_idempotent_write,
        });
    }

    AuthoritativeApplyReceipt::success(receipt_context, after_digest, report)
}

fn guarded_replay_receipt<T>(
    target: &T,
    plan: &MutationApplyPlan,
    before_digest: engine_core::StableDigest64,
    mode: MutationApplyMode,
    receipt_context: ApplyReceiptContext,
) -> Option<AuthoritativeApplyReceipt>
where
    T: MutationApplyTarget,
{
    if mode != MutationApplyMode::Replay || !plan.contains_non_idempotent_write {
        return None;
    }

    match target.applied_journal(plan.journal_digest()) {
        Some(journal) if journal.after_digest == before_digest => {
            Some(AuthoritativeApplyReceipt::success(
                receipt_context,
                before_digest,
                MutationApplyReport::all_applied(plan.operation_count),
            ))
        }
        Some(_) => Some(AuthoritativeApplyReceipt::validation_failed(
            receipt_context,
            MutationFailureReason::ReplayStateMismatch,
        )),
        None => Some(AuthoritativeApplyReceipt::validation_failed(
            receipt_context,
            MutationFailureReason::ReplayGuardRequired,
        )),
    }
}

fn missing_structural_target<T>(target: &T, plan: &MutationApplyPlan) -> bool
where
    T: MutationApplyTarget,
{
    plan.structural_components()
        .iter()
        .any(|component| !target.has_component(*component))
}
