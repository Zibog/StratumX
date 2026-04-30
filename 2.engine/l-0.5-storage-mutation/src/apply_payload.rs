use engine_core::{EngineCoreError, EngineCoreResult};
use engine_storage_access::WriteWindow;
use smallvec::SmallVec;

use crate::journal_digest::raw_payload_journal_digest;
use crate::{
    AppliedJournal, ApplyContext, ApplyFlags, ApplyPayload, ApplyReceiptContext,
    AuthoritativeApplyReceipt, ChangeSet, FamilyTag, MutationApplyMode, MutationApplyPlan,
    MutationApplyReport, MutationApplyTarget, MutationApplyTransaction, MutationBuffer,
    MutationFailureReason, RegionTag,
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
    context: ApplyContext,
) -> AuthoritativeApplyReceipt
where
    T: MutationApplyTarget + ?Sized,
{
    let before_digest = target.stable_state_digest();
    let fallback_journal_digest = raw_payload_journal_digest(payload);
    let fallback_context = ApplyReceiptContext {
        batch_id: context.batch_id,
        transaction_id: context.transaction_id,
        mode: context.mode,
        batch_order: payload.batch_order,
        family_tag: payload.family_tag,
        region_tag: payload.region_tag,
        before_digest,
        journal_digest: fallback_journal_digest,
    };

    let plan = match MutationApplyPlan::from_payload_with_policy(
        payload,
        context.conflict_policy,
        context.mode,
    ) {
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
        guarded_replay_receipt(target, &plan, before_digest, context.mode, receipt_context)
    {
        return receipt;
    }

    if missing_structural_target(target, &plan) {
        return AuthoritativeApplyReceipt::validation_failed(
            receipt_context,
            MutationFailureReason::MissingTarget,
        );
    }

    let mut transaction = target.begin_apply(context);
    if let Err(reason) = stage_plan(&mut transaction, &plan) {
        transaction.rollback();
        return AuthoritativeApplyReceipt::validation_failed(receipt_context, reason);
    }

    let after_digest = transaction.projected_state_digest();
    let report = MutationApplyReport::all_applied(plan.operation_count);

    if context.mode == MutationApplyMode::DryRun {
        transaction.rollback();
    } else {
        transaction.commit(AppliedJournal {
            journal_digest: plan.journal_digest(),
            before_digest,
            after_digest,
            contains_non_idempotent_write: plan.contains_non_idempotent_write,
        });
    }

    AuthoritativeApplyReceipt::success(receipt_context, after_digest, report)
}

fn stage_plan<T>(transaction: &mut T, plan: &MutationApplyPlan) -> Result<(), MutationFailureReason>
where
    T: crate::MutationApplyTransaction,
{
    for component in plan.structural_components() {
        transaction.stage_structural_removal(*component)?;
    }
    for write in plan.deferred_writes() {
        transaction.stage_write(write)?;
    }
    Ok(())
}

fn guarded_replay_receipt<T>(
    target: &T,
    plan: &MutationApplyPlan,
    before_digest: engine_core::StableDigest64,
    mode: MutationApplyMode,
    receipt_context: ApplyReceiptContext,
) -> Option<AuthoritativeApplyReceipt>
where
    T: MutationApplyTarget + ?Sized,
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
    T: MutationApplyTarget + ?Sized,
{
    plan.structural_components()
        .iter()
        .any(|component| !target.has_component(*component))
}
