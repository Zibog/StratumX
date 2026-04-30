use crate::DeliveryVerdict;
use engine_core::EngineCoreError;
use serde::{Deserialize, Serialize};

/// Network failure reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkFailureReason {
    InvalidSession,
    StaleSessionEpoch,
    InvalidLane,
    PacketTooLarge,
    DuplicatePacket,
    OutOfOrderPacket,
    AckOutOfWindow,
    AckRegression,
    ConnectionMismatch,
    ExpiredPacket,
}

impl NetworkFailureReason {
    const fn message(self) -> &'static str {
        match self {
            Self::InvalidSession => "session not active",
            Self::StaleSessionEpoch => "stale session epoch rejected",
            Self::InvalidLane => "packet descriptor violates lane policy",
            Self::PacketTooLarge => "packet exceeds transport packet ceiling",
            Self::DuplicatePacket => "packet sequence already processed",
            Self::OutOfOrderPacket => "packet sequence regressed",
            Self::AckOutOfWindow => "ack sequence exceeds sent window",
            Self::AckRegression => "ack sequence is not monotonic",
            Self::ConnectionMismatch => "envelope connection does not match session",
            Self::ExpiredPacket => "packet expired",
        }
    }

    pub(crate) fn as_engine_error(self) -> EngineCoreError {
        EngineCoreError::InvalidDescriptor(self.message())
    }
}

pub(crate) fn map_reject_reason_to_verdict(reason: NetworkFailureReason) -> DeliveryVerdict {
    match reason {
        NetworkFailureReason::InvalidSession => DeliveryVerdict::InvalidSession,
        NetworkFailureReason::StaleSessionEpoch => DeliveryVerdict::StaleSessionEpoch,
        NetworkFailureReason::InvalidLane => DeliveryVerdict::InvalidLane,
        NetworkFailureReason::PacketTooLarge
        | NetworkFailureReason::AckOutOfWindow
        | NetworkFailureReason::AckRegression
        | NetworkFailureReason::ConnectionMismatch => DeliveryVerdict::Rejected,
        NetworkFailureReason::DuplicatePacket => DeliveryVerdict::Duplicate,
        NetworkFailureReason::OutOfOrderPacket => DeliveryVerdict::OutOfOrder,
        NetworkFailureReason::ExpiredPacket => DeliveryVerdict::Expired,
    }
}
