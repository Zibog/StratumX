//! Legality gate validation functions
//!
//! Provides validation logic for compatibility verdicts and transport legality.

use sdk_compat::profiles::CompatProfileId;
use sdk_compat::verdicts::{
    CompatVerdictId, CompatibilityReasonCode, CompatibilityVerdict, LegalityRejection,
    LegalityRejectionReason, LegalityVerdict,
};
use sdk_compat::versions::{BridgeVersion, CompatDomain, CompatVersionId};
use sdk_compat::Capability;
use sdk_compat::CompatibilityProfile;
use transport_policies::{FramingKind, TransportPolicy};

use super::types::{GateDenyReason, LegalityGate, LegalityGateId, LegalityGateRegistry};

pub fn default_legality_gate_registry() -> LegalityGateRegistry {
    let mut registry = LegalityGateRegistry::new();
    for (i, domain) in CompatDomain::ALL.iter().enumerate() {
        let required = match domain {
            CompatDomain::World
            | CompatDomain::Material
            | CompatDomain::Animation
            | CompatDomain::Runtime => vec![Capability::Snapshots, Capability::Controls],
            CompatDomain::Terrain
            | CompatDomain::Destruction
            | CompatDomain::Wounds
            | CompatDomain::Capture => vec![Capability::Snapshots, Capability::Observations],
            CompatDomain::Weather
            | CompatDomain::Audio
            | CompatDomain::Society
            | CompatDomain::Ecology => vec![Capability::Observations],
            CompatDomain::Living => vec![Capability::Observations, Capability::Controls],
            CompatDomain::Tactics | CompatDomain::Diagnostics => {
                vec![Capability::Snapshots, Capability::Metrics]
            }
            CompatDomain::Photoreal | CompatDomain::Proof | CompatDomain::Build => {
                vec![Capability::Snapshots, Capability::ArtifactRefs]
            }
        };
        registry = registry.with_gate(LegalityGate::new(
            LegalityGateId(i as u64),
            domain.name(),
            *domain,
            required,
            GateDenyReason::MissingCapability(Capability::Snapshots),
        ));
    }
    registry
}

pub fn compatibility_verdict(
    presented_version: BridgeVersion,
    capabilities: &[Capability],
    profile: CompatibilityProfile,
    baseline_version: BridgeVersion,
) -> CompatibilityVerdict {
    let required: &[Capability] = match profile {
        CompatibilityProfile::ToolRuntime => &[Capability::Snapshots, Capability::Controls],
        CompatibilityProfile::EditorSurface => &[Capability::Snapshots, Capability::Observations],
        CompatibilityProfile::Automation => &[Capability::Snapshots, Capability::Metrics],
        CompatibilityProfile::Diagnostics => &[Capability::Observations, Capability::Metrics],
    };
    if presented_version.major < baseline_version.major {
        return CompatibilityVerdict::deny(
            CompatVerdictId(0),
            CompatProfileId(profile as u64 + 1),
            CompatVersionId(0),
            vec![CompatibilityReasonCode::VersionTooOld],
        );
    }
    let missing: Vec<_> = required
        .iter()
        .filter(|c| !capabilities.contains(c))
        .map(|c| CompatibilityReasonCode::MissingCapability(*c))
        .collect();
    if missing.is_empty() {
        CompatibilityVerdict::allow(
            CompatVerdictId(0),
            CompatProfileId(profile as u64 + 1),
            CompatVersionId(0),
        )
    } else {
        CompatibilityVerdict::deny(
            CompatVerdictId(0),
            CompatProfileId(profile as u64 + 1),
            CompatVersionId(0),
            missing,
        )
    }
}

pub fn transport_legality(
    policy: &TransportPolicy,
    payload_bytes: usize,
    includes_metric: bool,
    includes_artifact_ref: bool,
) -> Result<LegalityVerdict, LegalityRejection> {
    if payload_bytes > policy.max_payload_bytes {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::PayloadTooLarge {
                size: payload_bytes,
                max: policy.max_payload_bytes,
            },
        });
    }
    if includes_metric && !policy.allow_metrics {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::MetricNotAllowed {
                policy_name: format!("policy_{}", policy.transport_policy_id.0),
            },
        });
    }
    if includes_artifact_ref && !policy.allow_artifact_refs {
        return Err(LegalityRejection {
            verdict: LegalityVerdict::Illegal,
            reason: LegalityRejectionReason::ArtifactRefNotAllowed {
                policy_name: format!("policy_{}", policy.transport_policy_id.0),
            },
        });
    }
    match policy.framing_kind {
        FramingKind::MetricsOnly if payload_bytes > 0 && !includes_metric => {
            return Err(LegalityRejection {
                verdict: LegalityVerdict::Illegal,
                reason: LegalityRejectionReason::FramingViolation {
                    expected: "metrics".into(),
                    got: "payload".into(),
                },
            })
        }
        FramingKind::ArtifactOnly if !includes_artifact_ref => {
            return Err(LegalityRejection {
                verdict: LegalityVerdict::Illegal,
                reason: LegalityRejectionReason::FramingViolation {
                    expected: "artifact_ref".into(),
                    got: "other".into(),
                },
            })
        }
        _ => {}
    }
    Ok(LegalityVerdict::Legal)
}
