use engine_content::ContentManifest;
use engine_runtime::RuntimeProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkRole {
    LocalOnly,
    InteractiveHostAware,
    ListenHost,
    HeadlessHost,
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
