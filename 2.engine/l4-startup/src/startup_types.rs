use engine_content::ContentManifest;
use engine_core::{EngineCoreError, StableDigest64, StableDigestBuilder};
use engine_runtime::RuntimeProfile;
use serde::{Deserialize, Serialize};

pub type StartupResult<T> = Result<T, StartupFailure>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartupServiceId {
    Streaming,
    Residency,
    Memory,
    Transfer,
    Simulation,
    Networking,
    Modeling,
    Synthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRole {
    LocalOnly,
    InteractiveHostAware,
    ListenHost,
    HeadlessHost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartupFailureReason {
    InvalidProfileRoleCombination(RuntimeProfile, NetworkRole),
    MissingRequiredService(StartupServiceId),
    RuntimePackMissingPack,
    RuntimePackMissingLocator,
    RuntimePackLocatorEmpty,
    RuntimePackIdZero,
    RuntimePackChunkCountZero,
    RuntimePackDuplicateId,
    InvalidHeadlessLaunchProfile,
    InvalidRealtimeLaunchProfile,
    InvalidRealtimeRuntimeConfig,
}

impl StartupFailureReason {
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidProfileRoleCombination(
                RuntimeProfile::Headless20,
                NetworkRole::InteractiveHostAware,
            ) => "headless profile cannot bind interactive host-aware role",
            Self::InvalidProfileRoleCombination(
                RuntimeProfile::Interactive60,
                NetworkRole::HeadlessHost,
            ) => "interactive profile cannot bind headless host role",
            Self::InvalidProfileRoleCombination(
                RuntimeProfile::ListenHost60,
                NetworkRole::HeadlessHost,
            ) => "listen-host profile cannot bind headless host role",
            Self::InvalidProfileRoleCombination(_, _) => {
                "startup profile role combination is invalid"
            }
            Self::MissingRequiredService(StartupServiceId::Streaming) => {
                "missing required startup service: streaming"
            }
            Self::MissingRequiredService(StartupServiceId::Residency) => {
                "missing required startup service: residency"
            }
            Self::MissingRequiredService(StartupServiceId::Memory) => {
                "missing required startup service: memory"
            }
            Self::MissingRequiredService(StartupServiceId::Transfer) => {
                "missing required startup service: transfer"
            }
            Self::MissingRequiredService(StartupServiceId::Simulation) => {
                "missing required startup service: simulation"
            }
            Self::MissingRequiredService(StartupServiceId::Networking) => {
                "missing required startup service: networking"
            }
            Self::MissingRequiredService(StartupServiceId::Modeling) => {
                "missing required startup service: modeling"
            }
            Self::MissingRequiredService(StartupServiceId::Synthesis) => {
                "missing required startup service: synthesis"
            }
            Self::RuntimePackMissingPack => "runtime pack manifest requires at least one pack",
            Self::RuntimePackMissingLocator => {
                "runtime pack manifest requires at least one locator"
            }
            Self::RuntimePackLocatorEmpty => "runtime pack locator uri must be non-empty",
            Self::RuntimePackIdZero => "runtime pack id must be non-zero",
            Self::RuntimePackChunkCountZero => "runtime pack chunk count must be non-zero",
            Self::RuntimePackDuplicateId => "runtime pack id must be unique across manifests",
            Self::InvalidHeadlessLaunchProfile => "headless launch requires headless profile",
            Self::InvalidRealtimeLaunchProfile => "realtime launch requires non-headless profile",
            Self::InvalidRealtimeRuntimeConfig => "startup realtime runtime config is invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupFailure {
    pub reason: StartupFailureReason,
    pub digest: StableDigest64,
}

impl StartupFailure {
    pub fn for_reason(reason: StartupFailureReason) -> Self {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.startup.failure")
            .write_bytes(reason.message().as_bytes());
        Self {
            reason,
            digest: digest.finish(),
        }
    }
}

impl From<StartupFailure> for EngineCoreError {
    fn from(failure: StartupFailure) -> Self {
        EngineCoreError::InvalidDescriptor(failure.reason.message())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceWiring {
    pub streaming: bool,
    pub residency: bool,
    pub memory: bool,
    pub transfer: bool,
    pub simulation: bool,
    pub networking: bool,
    pub modeling: bool,
    pub synthesis: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupServiceWiringReceipt {
    pub profile_id: u64,
    pub required_service_count: usize,
    pub wired_service_count: usize,
    pub required_service_bits: u16,
    pub wired_service_bits: u16,
    pub deterministic_digest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupConfig {
    pub profile: RuntimeProfile,
    pub network_role: NetworkRole,
    pub runtime_manifests: Vec<ContentManifest>,
    pub service_wiring: ServiceWiring,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupReadyAssemblyDecisionSet {
    pub accepted: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeLaunchPlan {
    pub profile: RuntimeProfile,
    pub network_role: NetworkRole,
    pub runtime_pack_count: usize,
    pub service_wiring: ServiceWiring,
}
