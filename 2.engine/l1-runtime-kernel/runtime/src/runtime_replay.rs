use crate::runtime_budget::build_degrade_decision;
use crate::{
    BudgetEnvelope, DegradeDecision, DomainBudgetUsage, PressureBucket, RuntimeKernel, RuntimeMode,
    RuntimeProfile,
};
use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_world::ApplySegment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayInputFrame {
    pub tick: Tick,
    pub apply_segments: Vec<ApplySegment>,
    pub usage: DomainBudgetUsage,
    pub envelope: BudgetEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayOutputDigest {
    pub tick: Tick,
    pub world_digest: u64,
    pub runtime_digest: u64,
    pub degrade_decision: DegradeDecision,
}

impl RuntimeKernel {
    pub fn mode(&self) -> RuntimeMode {
        match self.profile() {
            RuntimeProfile::Headless20 => RuntimeMode::Headless,
            RuntimeProfile::Interactive60 | RuntimeProfile::ListenHost60 => RuntimeMode::Realtime,
        }
    }

    pub fn last_degrade_decision(&self) -> Option<&DegradeDecision> {
        self.last_degrade_decision.as_ref()
    }

    pub fn evaluate_budget(
        &self,
        envelope: &BudgetEnvelope,
        usage: &DomainBudgetUsage,
    ) -> EngineCoreResult<DegradeDecision> {
        build_degrade_decision(usage, envelope)
    }

    pub fn run_tick_with_budget(
        &mut self,
        envelope: BudgetEnvelope,
        usage: DomainBudgetUsage,
    ) -> EngineCoreResult<(crate::ExecutionResult, DegradeDecision)> {
        let decision = self.evaluate_budget(&envelope, &usage)?;
        self.last_degrade_decision = Some(decision.clone());
        if decision.bucket == PressureBucket::HardFail {
            return Err(EngineCoreError::InvalidDescriptor(
                "runtime budget entered hard-fail pressure bucket",
            ));
        }
        let result = self.run_tick()?;
        Ok((result, decision))
    }

    pub fn replay_frame(
        &mut self,
        input: ReplayInputFrame,
    ) -> EngineCoreResult<ReplayOutputDigest> {
        if input.tick != self.context().tick {
            return Err(EngineCoreError::InvalidDescriptor(
                "replay input tick must match runtime context tick",
            ));
        }
        for segment in input.apply_segments {
            self.enqueue_apply_segment(segment)?;
        }
        let (result, degrade_decision) = self.run_tick_with_budget(input.envelope, input.usage)?;
        Ok(ReplayOutputDigest {
            tick: result.tick,
            world_digest: self.world().deterministic_digest()?,
            runtime_digest: self.deterministic_digest()?,
            degrade_decision,
        })
    }
}
