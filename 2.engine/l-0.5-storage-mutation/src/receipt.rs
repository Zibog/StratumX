use engine_core::{StableDigest64, StableDigestBuilder};

use crate::{
    ApplyOutcome, ApplyTransactionId, FamilyTag, MutationApplyReport, MutationBatchId,
    MutationFailureReason, MutationOpStatus, RegionTag,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyReceiptContext {
    pub batch_id: MutationBatchId,
    pub transaction_id: ApplyTransactionId,
    pub batch_order: u64,
    pub family_tag: FamilyTag,
    pub region_tag: RegionTag,
    pub before_digest: StableDigest64,
    pub journal_digest: StableDigest64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeApplyReceipt {
    pub batch_id: MutationBatchId,
    pub transaction_id: ApplyTransactionId,
    pub batch_order: u64,
    pub family_tag: FamilyTag,
    pub region_tag: RegionTag,
    pub outcome: ApplyOutcome,
    pub before_digest: StableDigest64,
    pub after_digest: StableDigest64,
    pub journal_digest: StableDigest64,
    pub digest: StableDigest64,
}

impl AuthoritativeApplyReceipt {
    pub fn success(
        context: ApplyReceiptContext,
        after_digest: StableDigest64,
        report: MutationApplyReport,
    ) -> Self {
        Self::from_parts(context, after_digest, ApplyOutcome::Success(report))
    }

    pub fn validation_failed(context: ApplyReceiptContext, reason: MutationFailureReason) -> Self {
        Self::from_parts(
            context,
            context.before_digest,
            ApplyOutcome::ValidationFailed(reason),
        )
    }

    pub fn is_success(&self) -> bool {
        matches!(self.outcome, ApplyOutcome::Success(_))
    }

    fn from_parts(
        context: ApplyReceiptContext,
        after_digest: StableDigest64,
        outcome: ApplyOutcome,
    ) -> Self {
        let digest = compute_digest(&context, after_digest, &outcome);
        Self {
            batch_id: context.batch_id,
            transaction_id: context.transaction_id,
            batch_order: context.batch_order,
            family_tag: context.family_tag,
            region_tag: context.region_tag,
            outcome,
            before_digest: context.before_digest,
            after_digest,
            journal_digest: context.journal_digest,
            digest,
        }
    }
}

fn compute_digest(
    context: &ApplyReceiptContext,
    after_digest: StableDigest64,
    outcome: &ApplyOutcome,
) -> StableDigest64 {
    let mut builder = StableDigestBuilder::new();
    builder.write_bytes(b"engine_storage_mutation.receipt.v2");
    builder.write_u64(context.batch_id.0);
    builder.write_u64(context.transaction_id.0);
    builder.write_u64(context.batch_order);
    builder.write_u16(context.family_tag.0);
    builder.write_u32(context.region_tag.0);
    builder.write_u64(context.before_digest.0);
    builder.write_u64(after_digest.0);
    builder.write_u64(context.journal_digest.0);
    write_outcome(&mut builder, outcome);
    builder.finish()
}

fn write_outcome(builder: &mut StableDigestBuilder, outcome: &ApplyOutcome) {
    match outcome {
        ApplyOutcome::Success(report) => write_report(builder, report),
        ApplyOutcome::ValidationFailed(reason) => {
            builder.write_u8(0);
            builder.write_u8(reason.code());
        }
    }
}

fn write_report(builder: &mut StableDigestBuilder, report: &MutationApplyReport) {
    builder.write_u8(1);
    builder.write_u64(report.applied_count as u64);
    builder.write_u64(report.rejected_count as u64);
    builder.write_u64(report.operation_statuses.len() as u64);
    for status in &report.operation_statuses {
        match status {
            MutationOpStatus::Applied => builder.write_u8(1),
            MutationOpStatus::Rejected(reason) => builder.write_u8(0).write_u8(reason.code()),
        };
    }
}
