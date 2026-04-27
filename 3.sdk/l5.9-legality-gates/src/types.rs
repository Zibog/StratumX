//! Legality gate type definitions
//!
//! Defines types for legality gates, gate registries, and deny reasons.

use sdk_compat::verdicts::DomainRejectionReason;
use sdk_compat::versions::CompatDomain;
use sdk_compat::Capability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LegalityGateId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalityGate {
    pub legality_gate_id: LegalityGateId,
    pub gate_name: String,
    pub applies_to_domain: CompatDomain,
    pub required_capability_set: Vec<Capability>,
    pub deny_reason: GateDenyReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateDenyReason {
    MissingCapability(Capability),
    DomainIncompatible {
        domain: CompatDomain,
        reason: DomainRejectionReason,
    },
    VersionBelowMinimum,
    PolicyViolation,
}

impl LegalityGate {
    pub fn new(
        legality_gate_id: LegalityGateId,
        gate_name: &'static str,
        applies_to_domain: CompatDomain,
        required_capability_set: Vec<Capability>,
        deny_reason: GateDenyReason,
    ) -> Self {
        Self {
            legality_gate_id,
            gate_name: gate_name.to_string(),
            applies_to_domain,
            required_capability_set,
            deny_reason,
        }
    }

    pub fn check_capabilities(&self, capabilities: &[Capability]) -> Result<(), Vec<Capability>> {
        let missing: Vec<Capability> = self
            .required_capability_set
            .iter()
            .filter(|c| !capabilities.contains(c))
            .copied()
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalityGateRegistry {
    pub gates: Vec<LegalityGate>,
}

impl LegalityGateRegistry {
    pub fn new() -> Self {
        Self { gates: Vec::new() }
    }

    pub fn with_gate(mut self, gate: LegalityGate) -> Self {
        self.gates.push(gate);
        self
    }

    pub fn check_domain_gates(
        &self,
        domain: CompatDomain,
        capabilities: &[Capability],
    ) -> Result<(), Vec<(LegalityGateId, Vec<Capability>)>> {
        let failures: Vec<_> = self
            .gates
            .iter()
            .filter(|g| g.applies_to_domain == domain)
            .filter_map(|g| {
                g.check_capabilities(capabilities)
                    .err()
                    .map(|m| (g.legality_gate_id, m))
            })
            .collect();
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures)
        }
    }
}

impl Default for LegalityGateRegistry {
    fn default() -> Self {
        Self::new()
    }
}
