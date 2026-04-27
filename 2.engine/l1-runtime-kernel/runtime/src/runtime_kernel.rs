use crate::{
    ConnectionKey, ExecutionContext, ExecutionResult, PresentableFrame, PublicationRecord,
    RuntimeConfig, RuntimeDiagnostics, RuntimePhase, RuntimeProfile, TransferCompletion,
    APPLY_QUEUE_AGGREGATE_CEILING, APPLY_QUEUE_SEGMENT_CEILING,
    CONNECTION_PUBLICATION_BYTES_CEILING, CONNECTION_PUBLICATION_QUEUE_CEILING,
    PRESENTABLE_FRAME_QUEUE_CEILING, TRANSFER_COMPLETION_QUEUE_CEILING,
};
pub use engine_core::{EngineCoreError, EngineCoreResult};
use engine_world::{ApplySegment, WorldState};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub struct RuntimeKernel {
    world: WorldState,
    config: RuntimeConfig,
    phase: RuntimePhase,
    apply_queue: VecDeque<ApplySegment>,
    transfer_completion_queue: VecDeque<TransferCompletion>,
    connection_publication_queue: BTreeMap<ConnectionKey, VecDeque<PublicationRecord>>,
    presentable_frame_queue: VecDeque<PresentableFrame>,
    publication_order: u64,
}

impl RuntimeKernel {
    pub fn new(world: WorldState, config: RuntimeConfig) -> Self {
        Self {
            world,
            config,
            phase: RuntimePhase::Ingress,
            apply_queue: VecDeque::new(),
            transfer_completion_queue: VecDeque::new(),
            connection_publication_queue: BTreeMap::new(),
            presentable_frame_queue: VecDeque::new(),
            publication_order: 1,
        }
    }
    pub fn profile(&self) -> RuntimeProfile {
        self.config.profile
    }
    pub fn world(&self) -> &WorldState {
        &self.world
    }
    pub fn world_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }

    pub fn enqueue_apply_segment(&mut self, segment: ApplySegment) -> EngineCoreResult<()> {
        if segment.family_tags.len() > engine_world::MAX_FAMILY_FANOUT_PER_SEGMENT {
            return Err(EngineCoreError::InvalidDescriptor(
                "family fan-out exceeds canonical ceiling",
            ));
        }
        if self.apply_queue.len() >= APPLY_QUEUE_AGGREGATE_CEILING {
            return Err(EngineCoreError::InvalidDescriptor(
                "apply queue exceeds aggregate canonical ceiling",
            ));
        }
        self.apply_queue.push_back(segment);
        Ok(())
    }

    pub fn enqueue_transfer_completion(
        &mut self,
        completion: TransferCompletion,
    ) -> EngineCoreResult<()> {
        if self.transfer_completion_queue.len() >= TRANSFER_COMPLETION_QUEUE_CEILING {
            return Err(EngineCoreError::InvalidDescriptor(
                "transfer completion queue exceeds canonical ceiling",
            ));
        }
        self.transfer_completion_queue.push_back(completion);
        Ok(())
    }

    pub fn enqueue_connection_publication(
        &mut self,
        key: ConnectionKey,
        payload: Vec<u8>,
    ) -> EngineCoreResult<u64> {
        let queue = self.connection_publication_queue.entry(key).or_default();
        let queued_bytes: usize = queue.iter().map(|record| record.bytes.len()).sum();
        if queue.len() >= CONNECTION_PUBLICATION_QUEUE_CEILING {
            return Err(EngineCoreError::InvalidDescriptor(
                "per-connection publication queue exceeds canonical ceiling",
            ));
        }
        if queued_bytes.saturating_add(payload.len()) > CONNECTION_PUBLICATION_BYTES_CEILING {
            return Err(EngineCoreError::InvalidDescriptor(
                "per-connection queued bytes exceed canonical ceiling",
            ));
        }
        let order = self.publication_order;
        self.publication_order = self.publication_order.saturating_add(1);
        queue.push_back(PublicationRecord {
            order,
            bytes: payload,
        });
        Ok(order)
    }

    pub fn enqueue_presentable_frame(&mut self, frame: PresentableFrame) -> EngineCoreResult<()> {
        if self.config.profile == RuntimeProfile::Headless20 {
            return Err(EngineCoreError::InvalidDescriptor(
                "headless profile may not own presentation queue",
            ));
        }
        if self.presentable_frame_queue.len() >= PRESENTABLE_FRAME_QUEUE_CEILING {
            return Err(EngineCoreError::InvalidDescriptor(
                "presentable frame queue exceeds canonical ceiling",
            ));
        }
        self.presentable_frame_queue.push_back(frame);
        Ok(())
    }

    pub fn diagnostics(&self) -> RuntimeDiagnostics {
        RuntimeDiagnostics {
            last_phase: self.phase,
            apply_queue_depth: self.apply_queue.len(),
            transfer_completion_queue_depth: self.transfer_completion_queue.len(),
            connection_count: self.connection_publication_queue.len(),
            presentable_frame_depth: self.presentable_frame_queue.len(),
        }
    }

    pub fn context(&self) -> ExecutionContext {
        ExecutionContext {
            tick: self.world.read_model().tick,
            phase: self.phase,
        }
    }

    pub fn run_tick(&mut self) -> EngineCoreResult<ExecutionResult> {
        self.phase = RuntimePhase::Ingress;
        self.phase = RuntimePhase::Read;
        self.phase = RuntimePhase::Compute;
        self.phase = RuntimePhase::Resource;
        self.phase = RuntimePhase::AuthoritySync;
        self.phase = RuntimePhase::Stage;
        let max_segments = self
            .config
            .max_apply_segments_per_tick
            .min(engine_world::MAX_SEGMENTS_PER_TICK)
            .min(APPLY_QUEUE_SEGMENT_CEILING);
        let mut segments = Vec::new();
        while segments.len() < max_segments {
            match self.apply_queue.pop_front() {
                Some(segment) => segments.push(segment),
                None => break,
            }
        }
        self.phase = RuntimePhase::Apply;
        self.world.apply(&segments, self.config.publish_passes)?;
        self.phase = RuntimePhase::Egress;
        let presented_frame = if self.config.profile == RuntimeProfile::Headless20 {
            false
        } else {
            self.presentable_frame_queue.pop_front().is_some()
        };
        for queue in self.connection_publication_queue.values_mut() {
            queue.clear();
        }
        self.transfer_completion_queue.clear();
        self.phase = RuntimePhase::Diagnostics;
        Ok(ExecutionResult {
            tick: self.world.read_model().tick,
            applied_segments: segments.len(),
            published_records: 0,
            presented_frame,
        })
    }
}
