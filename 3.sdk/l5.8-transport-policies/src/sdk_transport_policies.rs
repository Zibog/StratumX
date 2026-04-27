//! SDK Transport Policies — transport, rate, reliability, and scope policies.

use sdk_compat::BridgeVersion;
use sdk_compat::CompatibilityProfile;
use serde::{Deserialize, Serialize};

pub const CANONICAL_LEVEL: &str = "L5.8";
pub const MAX_PACKET_BYTES: usize = 16 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TransportPolicyId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FramingKind {
    OrderedControl,
    BoundedPreview,
    MetricsOnly,
    ArtifactOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetryClass {
    None,
    Bounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RateTier {
    Unlimited,
    Throttled { max_packets_per_second: u32 },
    Burst { burst_max: u32, sustained_avg: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PriorityLevel {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Background = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryScope {
    PointToPoint,
    Broadcast,
    Multicast,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicy {
    pub transport_policy_id: TransportPolicyId,
    pub framing_kind: FramingKind,
    pub max_payload_bytes: usize,
    pub retry_class: RetryClass,
    pub batching_window_ticks: u32,
    pub compat_profile: Option<CompatibilityProfile>,
    pub compat_version: Option<BridgeVersion>,
    pub allow_metrics: bool,
    pub allow_artifact_refs: bool,
    pub rate_tier: RateTier,
    pub priority: PriorityLevel,
    pub delivery_scope: DeliveryScope,
}

impl TransportPolicy {
    pub fn new(
        transport_policy_id: TransportPolicyId,
        framing_kind: FramingKind,
        max_payload_bytes: usize,
        retry_class: RetryClass,
        batching_window_ticks: u32,
    ) -> Self {
        Self {
            transport_policy_id,
            framing_kind,
            max_payload_bytes,
            retry_class,
            batching_window_ticks,
            compat_profile: None,
            compat_version: None,
            allow_metrics: false,
            allow_artifact_refs: false,
            rate_tier: RateTier::Unlimited,
            priority: PriorityLevel::Normal,
            delivery_scope: DeliveryScope::PointToPoint,
        }
    }

    pub fn with_compat_profile(mut self, profile: CompatibilityProfile) -> Self {
        self.compat_profile = Some(profile);
        self
    }

    pub fn with_compat_version(mut self, version: BridgeVersion) -> Self {
        self.compat_version = Some(version);
        self
    }

    pub fn with_metrics_allowed(mut self) -> Self {
        self.allow_metrics = true;
        self
    }

    pub fn with_artifact_refs_allowed(mut self) -> Self {
        self.allow_artifact_refs = true;
        self
    }

    pub fn with_rate_tier(mut self, tier: RateTier) -> Self {
        self.rate_tier = tier;
        self
    }

    pub fn with_priority(mut self, priority: PriorityLevel) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_delivery_scope(mut self, scope: DeliveryScope) -> Self {
        self.delivery_scope = scope;
        self
    }

    pub fn is_payload_legal(&self, size: usize) -> bool {
        size <= self.max_payload_bytes
    }
}

pub fn default_transport_policy(framing_kind: FramingKind) -> TransportPolicy {
    match framing_kind {
        FramingKind::OrderedControl => TransportPolicy::new(
            TransportPolicyId(1),
            framing_kind,
            MAX_PACKET_BYTES,
            RetryClass::Bounded,
            1,
        )
        .with_priority(PriorityLevel::Critical)
        .with_rate_tier(RateTier::Unlimited),
        FramingKind::BoundedPreview => TransportPolicy::new(
            TransportPolicyId(2),
            framing_kind,
            MAX_PACKET_BYTES / 2,
            RetryClass::Bounded,
            2,
        )
        .with_artifact_refs_allowed()
        .with_priority(PriorityLevel::Normal)
        .with_rate_tier(RateTier::Throttled {
            max_packets_per_second: 60,
        }),
        FramingKind::MetricsOnly => {
            TransportPolicy::new(TransportPolicyId(3), framing_kind, 0, RetryClass::None, 1)
                .with_metrics_allowed()
                .with_priority(PriorityLevel::Low)
                .with_rate_tier(RateTier::Burst {
                    burst_max: 120,
                    sustained_avg: 30,
                })
        }
        FramingKind::ArtifactOnly => {
            TransportPolicy::new(TransportPolicyId(4), framing_kind, 128, RetryClass::None, 1)
                .with_artifact_refs_allowed()
                .with_priority(PriorityLevel::High)
                .with_rate_tier(RateTier::Throttled {
                    max_packets_per_second: 10,
                })
        }
    }
}

/// Typed transport policy rejection payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPolicyRejection {
    pub policy_id: TransportPolicyId,
    pub reason: TransportPolicyRejectionReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportPolicyRejectionReason {
    PayloadTooLarge { size: usize, max: usize },
    MetricsNotAllowed,
    ArtifactRefsNotAllowed,
    RetryExhausted,
    RateLimited,
    PriorityDenied,
}
