use super::state::WorldState;
use crate::{
    ApplySegment, WorldSnapshot, MAX_FAMILY_FANOUT_PER_SEGMENT, MAX_PUBLISH_PASSES,
    MAX_SEGMENTS_PER_TICK,
};
use engine_core::{EngineCoreError, EngineCoreResult, Tick};

impl WorldState {
    pub fn snapshot(&self, segment_count: usize) -> WorldSnapshot {
        WorldSnapshot {
            tick: self.tick,
            epoch: self.epoch,
            segment_count,
        }
    }

    pub fn snapshot_bytes(&self, segment_count: usize) -> EngineCoreResult<Vec<u8>> {
        bincode::serialize(&self.snapshot(segment_count))
            .map_err(|_| EngineCoreError::InvalidDescriptor("snapshot serialization must succeed"))
    }

    pub fn apply(
        &mut self,
        segments: &[ApplySegment],
        publish_passes: usize,
    ) -> EngineCoreResult<()> {
        if segments.len() > MAX_SEGMENTS_PER_TICK {
            return Err(EngineCoreError::InvalidDescriptor(
                "segment count exceeds canonical ceiling",
            ));
        }
        if publish_passes > MAX_PUBLISH_PASSES {
            return Err(EngineCoreError::InvalidDescriptor(
                "publish passes exceed canonical ceiling",
            ));
        }
        for segment in segments {
            if segment.family_tags.len() > MAX_FAMILY_FANOUT_PER_SEGMENT {
                return Err(EngineCoreError::InvalidDescriptor(
                    "family fan-out exceeds canonical ceiling",
                ));
            }
        }
        self.tick = Tick(self.tick.0.saturating_add(1));
        self.epoch = self.epoch.saturating_add(1);
        Ok(())
    }
}
