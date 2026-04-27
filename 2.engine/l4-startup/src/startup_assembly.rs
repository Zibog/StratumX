use crate::{NetworkRole, RuntimeLaunchPlan, StartupConfig, StartupReadyAssemblyDecisionSet};
use engine_core::{EngineCoreError, EngineCoreResult};
use engine_runtime::RuntimeProfile;
use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

#[derive(Debug, Clone)]
pub struct StartupAssembly {
    config: StartupConfig,
}

impl StartupAssembly {
    pub fn new(config: StartupConfig) -> Self {
        Self { config }
    }

    fn validation_reason(&self) -> Option<&'static str> {
        match (self.config.profile, self.config.network_role) {
            (RuntimeProfile::Headless20, NetworkRole::InteractiveHostAware) => {
                Some("headless profile cannot bind interactive host-aware role")
            }
            (RuntimeProfile::Interactive60, NetworkRole::HeadlessHost) => {
                Some("interactive profile cannot bind headless host role")
            }
            (RuntimeProfile::ListenHost60, NetworkRole::HeadlessHost) => {
                Some("listen-host profile cannot bind headless host role")
            }
            _ => None,
        }
    }

    pub fn validate(&self) -> StartupReadyAssemblyDecisionSet {
        let reasons = self
            .validation_reason()
            .map(|reason| vec![reason.to_string()])
            .unwrap_or_default();
        StartupReadyAssemblyDecisionSet {
            accepted: reasons.is_empty(),
            reasons,
        }
    }

    pub fn runtime_launch_plan(&self) -> EngineCoreResult<RuntimeLaunchPlan> {
        if self.validation_reason().is_some() {
            return Err(EngineCoreError::InvalidDescriptor(
                "startup validation failed",
            ));
        }
        let runtime_pack_count: usize = self
            .config
            .runtime_manifests
            .iter()
            .map(|m| m.packs.len())
            .sum();
        Ok(RuntimeLaunchPlan {
            profile: self.config.profile,
            network_role: self.config.network_role,
            runtime_pack_count,
            service_wiring: self.config.service_wiring,
        })
    }

    pub fn launch_headless(&self, world: WorldState) -> EngineCoreResult<HeadlessRuntimeProfile> {
        if self.config.profile != RuntimeProfile::Headless20 {
            return Err(EngineCoreError::InvalidDescriptor(
                "headless launch requires headless profile",
            ));
        }
        self.runtime_launch_plan()?;
        Ok(HeadlessRuntimeProfile::new(
            world,
            HeadlessRuntimeConfig {
                snapshot_segment_count: 0,
                emit_snapshot_bytes: false,
            },
        ))
    }

    pub fn launch_realtime(&self, world: WorldState) -> EngineCoreResult<RealtimeRuntimeProfile> {
        if self.config.profile == RuntimeProfile::Headless20 {
            return Err(EngineCoreError::InvalidDescriptor(
                "realtime launch requires non-headless profile",
            ));
        }
        self.runtime_launch_plan()?;
        RealtimeRuntimeProfile::new(
            world,
            RealtimeRuntimeConfig {
                target_fps: 60,
                visibility_freshness_frames: 1,
                enqueue_presentable_frames: true,
            },
        )
    }
}
