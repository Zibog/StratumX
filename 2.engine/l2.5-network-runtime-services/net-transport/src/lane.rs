use crate::{NetworkFailureReason, NetworkLaneId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketDescriptor {
    pub reliable: bool,
    pub ordered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketLane {
    Control,
    State,
    Bulk,
}

impl PacketLane {
    pub fn as_lane_id(self) -> NetworkLaneId {
        match self {
            Self::Control => NetworkLaneId(0),
            Self::State => NetworkLaneId(1),
            Self::Bulk => NetworkLaneId(2),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SequenceAcceptancePolicy {
    #[default]
    MonotonicAllowGaps,
    ContiguousOnly,
}

impl SequenceAcceptancePolicy {
    pub fn validate(
        self,
        last_processed_sequence: u64,
        sequence: u64,
    ) -> Result<(), NetworkFailureReason> {
        if sequence == last_processed_sequence {
            return Err(NetworkFailureReason::DuplicatePacket);
        }
        if sequence < last_processed_sequence {
            return Err(NetworkFailureReason::OutOfOrderPacket);
        }
        if matches!(self, Self::ContiguousOnly)
            && sequence != last_processed_sequence.saturating_add(1)
        {
            return Err(NetworkFailureReason::OutOfOrderPacket);
        }
        Ok(())
    }
}

/// Lane policy enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanePolicy {
    pub requires_reliable: bool,
    pub requires_ordered: bool,
}

impl LanePolicy {
    pub const CONTROL: Self = Self {
        requires_reliable: true,
        requires_ordered: true,
    };
    pub const STATE: Self = Self {
        requires_reliable: true,
        requires_ordered: false,
    };
    pub const BULK: Self = Self {
        requires_reliable: false,
        requires_ordered: false,
    };

    pub fn for_lane(lane: PacketLane) -> Self {
        match lane {
            PacketLane::Control => Self::CONTROL,
            PacketLane::State => Self::STATE,
            PacketLane::Bulk => Self::BULK,
        }
    }

    pub fn validate(&self, descriptor: &PacketDescriptor) -> Result<(), NetworkFailureReason> {
        if self.requires_reliable && !descriptor.reliable {
            return Err(NetworkFailureReason::InvalidLane);
        }
        if self.requires_ordered && !descriptor.ordered {
            return Err(NetworkFailureReason::InvalidLane);
        }
        Ok(())
    }
}
