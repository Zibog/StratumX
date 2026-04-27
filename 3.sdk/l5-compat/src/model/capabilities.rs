//! Compatibility capability types
//! Merged from l5.5-compat-capabilities
//!
//! Defines feature capabilities, default states, and capability facts
//! for SDK compatibility negotiation across all 18 domain families.

use serde::{Deserialize, Serialize};

/// Opaque identifier for a capability entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompatCapabilityId(pub u64);

/// Core SDK capabilities that a peer may support.
///
/// Cadence: evaluated once per session negotiation.
/// Delivery guarantee: synchronous — capability set must be agreed upon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Capability {
    Snapshots,
    Observations,
    Metrics,
    ArtifactRefs,
    Controls,
}

impl Capability {
    pub const ALL: &'static [Self] = &[
        Self::Snapshots,
        Self::Observations,
        Self::Metrics,
        Self::ArtifactRefs,
        Self::Controls,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Snapshots => "snapshots",
            Self::Observations => "observations",
            Self::Metrics => "metrics",
            Self::ArtifactRefs => "artifact_refs",
            Self::Controls => "controls",
        }
    }
}

/// Default state for a capability when not explicitly negotiated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityDefaultState {
    Allowed,
    Denied,
}

/// A fact about a specific capability.
///
/// Cadence: loaded once during startup negotiation.
/// Delivery guarantee: static — part of the capability registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityFact {
    pub capability_id: CompatCapabilityId,
    pub capability_name: String,
    pub default_state: CapabilityDefaultState,
    pub deprecation_note: Option<String>,
}

impl CapabilityFact {
    pub fn new(
        capability_id: CompatCapabilityId,
        capability_name: &'static str,
        default_state: CapabilityDefaultState,
    ) -> Self {
        Self {
            capability_id,
            capability_name: capability_name.to_string(),
            default_state,
            deprecation_note: None,
        }
    }

    pub fn with_deprecation_note(mut self, note: &'static str) -> Self {
        self.deprecation_note = Some(note.to_string());
        self
    }
}

/// Domain-specific capability facts per domain family.
///
/// Each domain may require specific capabilities beyond the base set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainCapabilityFact {
    pub domain: crate::model::versions::CompatDomain,
    pub required_capabilities: Vec<Capability>,
    pub optional_capabilities: Vec<Capability>,
}

/// Typed capability rejection payload.
///
/// Replaces generic string errors when a capability is missing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRejection {
    pub missing_capabilities: Vec<Capability>,
    pub domain: Option<crate::model::versions::CompatDomain>,
    pub message: String,
}

/// Canonical capability registry.
///
/// Provides typed facts for every capability in the SDK.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRegistry {
    pub capabilities: Vec<CapabilityFact>,
    pub domain_capabilities: Vec<DomainCapabilityFact>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
            domain_capabilities: Vec::new(),
        }
    }

    pub fn with_capability(mut self, fact: CapabilityFact) -> Self {
        self.capabilities.push(fact);
        self
    }

    pub fn with_domain_capabilities(mut self, fact: DomainCapabilityFact) -> Self {
        self.domain_capabilities.push(fact);
        self
    }

    /// Check if a set of presented capabilities satisfies the required set for a domain.
    pub fn check_domain_capabilities(
        &self,
        domain: crate::model::versions::CompatDomain,
        presented: &[Capability],
    ) -> Result<(), CapabilityRejection> {
        let domain_fact = self.domain_capabilities.iter().find(|d| d.domain == domain);

        let required = match domain_fact {
            Some(d) => &d.required_capabilities,
            None => &Vec::new(),
        };

        let missing: Vec<Capability> = required
            .iter()
            .filter(|c| !presented.contains(c))
            .copied()
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(CapabilityRejection {
                missing_capabilities: missing,
                domain: Some(domain),
                message: "missing required capabilities for domain".to_string(),
            })
        }
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
