use crate::startup_validation::service_wiring_bitset;
use crate::{
    RuntimeLaunchPlan, StartupConfig, StartupFailure, StartupFailureReason,
    StartupReadyAssemblyDecisionSet, StartupResult,
};
use engine_core::{EngineCoreResult, StableDigestBuilder};
use engine_runtime::RuntimeProfile;
use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

#[derive(Debug, Clone)]
pub struct StartupAssembly {
    pub(crate) config: StartupConfig,
}

impl StartupAssembly {
    pub fn new(config: StartupConfig) -> Self {
        Self { config }
    }

    pub fn validate(&self) -> StartupReadyAssemblyDecisionSet {
        let reasons = self
            .validation_reasons()
            .into_iter()
            .map(|reason| reason.message().to_string())
            .collect::<Vec<_>>();
        StartupReadyAssemblyDecisionSet {
            accepted: reasons.is_empty(),
            reasons,
        }
    }

    pub fn runtime_launch_plan(&self) -> StartupResult<RuntimeLaunchPlan> {
        if let Some(reason) = self.validation_reasons().into_iter().next() {
            return Err(StartupFailure::for_reason(reason));
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

    pub fn service_wiring_receipt(&self) -> StartupResult<crate::StartupServiceWiringReceipt> {
        if let Some(reason) = self.validation_reasons().into_iter().next() {
            return Err(StartupFailure::for_reason(reason));
        }

        let wiring = &self.config.service_wiring;
        let service_bits = service_wiring_bitset(*wiring);
        let required_service_bits = self.required_service_bits();
        let wired_count = service_bits.count_ones() as usize;
        let required_count = required_service_bits.count_ones() as usize;
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.startup.service_wiring")
            .write_u8(self.config.profile as u8)
            .write_u8(self.config.network_role as u8)
            .write_u64(required_count as u64)
            .write_u64(wired_count as u64)
            .write_u16(required_service_bits)
            .write_u16(service_bits);

        Ok(crate::StartupServiceWiringReceipt {
            profile_id: self.config.profile as u64,
            required_service_count: required_count,
            wired_service_count: wired_count,
            required_service_bits,
            wired_service_bits: service_bits,
            deterministic_digest: digest.finish().0,
        })
    }

    pub fn launch_headless(&self, world: WorldState) -> StartupResult<HeadlessRuntimeProfile> {
        if self.config.profile != RuntimeProfile::Headless20 {
            return Err(StartupFailure::for_reason(
                StartupFailureReason::InvalidHeadlessLaunchProfile,
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

    pub fn launch_realtime(&self, world: WorldState) -> StartupResult<RealtimeRuntimeProfile> {
        if self.config.profile == RuntimeProfile::Headless20 {
            return Err(StartupFailure::for_reason(
                StartupFailureReason::InvalidRealtimeLaunchProfile,
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
        .map_err(|_| StartupFailure::for_reason(StartupFailureReason::InvalidRealtimeRuntimeConfig))
    }

    pub fn runtime_launch_plan_engine_result(&self) -> EngineCoreResult<RuntimeLaunchPlan> {
        self.runtime_launch_plan().map_err(Into::into)
    }

    pub fn service_wiring_receipt_engine_result(
        &self,
    ) -> EngineCoreResult<crate::StartupServiceWiringReceipt> {
        self.service_wiring_receipt().map_err(Into::into)
    }
}
