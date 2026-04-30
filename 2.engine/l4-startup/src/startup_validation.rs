use crate::startup_assembly::StartupAssembly;
use crate::{NetworkRole, ServiceWiring, StartupFailureReason, StartupServiceId};
use engine_runtime::RuntimeProfile;
use std::collections::BTreeSet;

const STREAMING_BIT: u16 = 1 << 0;
const RESIDENCY_BIT: u16 = 1 << 1;
const MEMORY_BIT: u16 = 1 << 2;
const TRANSFER_BIT: u16 = 1 << 3;
const SIMULATION_BIT: u16 = 1 << 4;
const NETWORKING_BIT: u16 = 1 << 5;
const MODELING_BIT: u16 = 1 << 6;
const SYNTHESIS_BIT: u16 = 1 << 7;

impl StartupAssembly {
    pub(crate) fn validation_reasons(&self) -> Vec<StartupFailureReason> {
        let mut reasons = Vec::new();
        if let Some(reason) = self.profile_role_reason() {
            reasons.push(reason);
        }
        reasons.extend(self.missing_required_service_reasons());
        if let Some(reason) = self.runtime_pack_compatibility_reason() {
            reasons.push(reason);
        }
        reasons
    }

    fn profile_role_reason(&self) -> Option<StartupFailureReason> {
        match (self.config.profile, self.config.network_role) {
            (RuntimeProfile::Headless20, NetworkRole::InteractiveHostAware)
            | (RuntimeProfile::Interactive60, NetworkRole::HeadlessHost)
            | (RuntimeProfile::ListenHost60, NetworkRole::HeadlessHost) => {
                Some(StartupFailureReason::InvalidProfileRoleCombination(
                    self.config.profile,
                    self.config.network_role,
                ))
            }
            _ => None,
        }
    }

    pub(crate) fn required_service_bits(&self) -> u16 {
        let mut bits = STREAMING_BIT | RESIDENCY_BIT | MEMORY_BIT | TRANSFER_BIT | SIMULATION_BIT;
        if self.config.network_role != NetworkRole::LocalOnly {
            bits |= NETWORKING_BIT;
        }
        if self.config.profile != RuntimeProfile::Headless20 {
            bits |= SYNTHESIS_BIT;
        }
        if !self.config.runtime_manifests.is_empty() {
            bits |= MODELING_BIT;
        }
        bits
    }

    fn missing_required_service_reasons(&self) -> Vec<StartupFailureReason> {
        let required_bits = self.required_service_bits();
        let wiring_bits = service_wiring_bitset(self.config.service_wiring);
        let mut reasons = Vec::new();
        for (bit, reason) in [
            (STREAMING_BIT, StartupServiceId::Streaming),
            (RESIDENCY_BIT, StartupServiceId::Residency),
            (MEMORY_BIT, StartupServiceId::Memory),
            (TRANSFER_BIT, StartupServiceId::Transfer),
            (SIMULATION_BIT, StartupServiceId::Simulation),
            (NETWORKING_BIT, StartupServiceId::Networking),
            (MODELING_BIT, StartupServiceId::Modeling),
            (SYNTHESIS_BIT, StartupServiceId::Synthesis),
        ] {
            if required_bits & bit != 0 && wiring_bits & bit == 0 {
                reasons.push(StartupFailureReason::MissingRequiredService(reason));
            }
        }
        reasons
    }

    fn runtime_pack_compatibility_reason(&self) -> Option<StartupFailureReason> {
        let mut pack_ids = BTreeSet::new();
        for manifest in &self.config.runtime_manifests {
            if manifest.packs.is_empty() {
                return Some(StartupFailureReason::RuntimePackMissingPack);
            }
            if manifest.locators.is_empty() {
                return Some(StartupFailureReason::RuntimePackMissingLocator);
            }
            for locator in &manifest.locators {
                if locator.uri.trim().is_empty() {
                    return Some(StartupFailureReason::RuntimePackLocatorEmpty);
                }
            }
            for pack in &manifest.packs {
                if pack.pack_id == 0 {
                    return Some(StartupFailureReason::RuntimePackIdZero);
                }
                if pack.chunk_count == 0 {
                    return Some(StartupFailureReason::RuntimePackChunkCountZero);
                }
                if !pack_ids.insert(pack.pack_id) {
                    return Some(StartupFailureReason::RuntimePackDuplicateId);
                }
            }
        }
        None
    }
}

pub(crate) fn service_wiring_bitset(wiring: ServiceWiring) -> u16 {
    let mut bits = 0u16;
    if wiring.streaming {
        bits |= STREAMING_BIT;
    }
    if wiring.residency {
        bits |= RESIDENCY_BIT;
    }
    if wiring.memory {
        bits |= MEMORY_BIT;
    }
    if wiring.transfer {
        bits |= TRANSFER_BIT;
    }
    if wiring.simulation {
        bits |= SIMULATION_BIT;
    }
    if wiring.networking {
        bits |= NETWORKING_BIT;
    }
    if wiring.modeling {
        bits |= MODELING_BIT;
    }
    if wiring.synthesis {
        bits |= SYNTHESIS_BIT;
    }
    bits
}
