use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationFailureReason {
    InvalidBatchOrder,
    EmptyChangeset,
    InvalidHandle,
    StaleGeneration,
    DuplicateOperation,
    ConflictingOperation,
    MissingTarget,
    IllegalOperation,
    ReplayGuardRequired,
    ReplayStateMismatch,
}

impl MutationFailureReason {
    pub(crate) fn code(self) -> u8 {
        match self {
            Self::InvalidBatchOrder => 1,
            Self::EmptyChangeset => 2,
            Self::InvalidHandle => 3,
            Self::StaleGeneration => 4,
            Self::DuplicateOperation => 5,
            Self::ConflictingOperation => 6,
            Self::MissingTarget => 7,
            Self::IllegalOperation => 8,
            Self::ReplayGuardRequired => 9,
            Self::ReplayStateMismatch => 10,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationOpStatus {
    Applied,
    Rejected(MutationFailureReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationApplyReport {
    pub applied_count: usize,
    pub rejected_count: usize,
    pub operation_statuses: SmallVec<[MutationOpStatus; 16]>,
}

impl MutationApplyReport {
    pub fn all_applied(count: usize) -> Self {
        Self {
            applied_count: count,
            rejected_count: 0,
            operation_statuses: (0..count).map(|_| MutationOpStatus::Applied).collect(),
        }
    }

    pub fn all_rejected(count: usize, reason: MutationFailureReason) -> Self {
        Self {
            applied_count: 0,
            rejected_count: count,
            operation_statuses: (0..count)
                .map(|_| MutationOpStatus::Rejected(reason))
                .collect(),
        }
    }

    pub fn has_rejections(&self) -> bool {
        self.rejected_count > 0
    }

    pub fn all_succeeded(&self) -> bool {
        self.rejected_count == 0 && self.applied_count > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyOutcome {
    Success(MutationApplyReport),
    ValidationFailed(MutationFailureReason),
}
