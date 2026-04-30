use super::state::WorldState;
use crate::{AppliedSegmentRecord, WorldCausalSummary, WorldSnapshot};
use engine_core::{EngineCoreError, EngineCoreResult, StableDigestBuilder};
use engine_world_spatial::{FarPhenomenonTrackRef, RegionFrameRef};

impl WorldState {
    pub fn snapshot(&self, segment_count: usize) -> WorldSnapshot {
        WorldSnapshot {
            world_id: self.manifest.world_id,
            tick: self.tick,
            epoch: self.epoch,
            segment_count,
        }
    }

    pub fn snapshot_bytes(&self, segment_count: usize) -> EngineCoreResult<Vec<u8>> {
        bincode::serialize(&self.snapshot(segment_count))
            .map_err(|_| EngineCoreError::InvalidDescriptor("snapshot serialization must succeed"))
    }

    pub fn deterministic_digest(&self) -> EngineCoreResult<u64> {
        let snapshot = self.snapshot(self.last_apply_journal.segment_count);
        let mut digest = StableDigestBuilder::new();
        digest.write_bytes(b"engine.world.state");
        write_snapshot_digest(&mut digest, &snapshot);
        write_apply_journal_digest(&mut digest, &self.last_apply_journal);
        write_causal_summary_digest(&mut digest, &self.last_causal_summary);
        Ok(digest.finish().0)
    }
}

fn write_snapshot_digest(builder: &mut StableDigestBuilder, snapshot: &WorldSnapshot) {
    builder
        .write_u64(snapshot.world_id.0)
        .write_u64(snapshot.tick.0)
        .write_u64(snapshot.epoch)
        .write_u64(snapshot.segment_count as u64);
}

fn write_apply_journal_digest(
    builder: &mut StableDigestBuilder,
    journal: &crate::WorldApplyJournal,
) {
    builder
        .write_u64(journal.tick.0)
        .write_u64(journal.epoch)
        .write_u64(journal.publish_passes as u64)
        .write_u64(journal.segment_count as u64)
        .write_u64(journal.segments.len() as u64);
    for segment in &journal.segments {
        write_segment_record_digest(builder, segment);
    }
}

fn write_segment_record_digest(builder: &mut StableDigestBuilder, segment: &AppliedSegmentRecord) {
    write_region_key_digest(builder, segment.region_key);
    builder.write_u64(segment.family_tags.len() as u64);
    for family_tag in &segment.family_tags {
        builder.write_u16(*family_tag);
    }
}

fn write_causal_summary_digest(builder: &mut StableDigestBuilder, summary: &WorldCausalSummary) {
    builder
        .write_u64(summary.region_keys.len() as u64)
        .write_u64(summary.family_tags.len() as u64)
        .write_u64(summary.publish_passes as u64)
        .write_bool(summary.near_region_frame_ref.is_some())
        .write_u64(summary.far_phenomenon_track_refs.len() as u64)
        .write_u8(summary.precision_zone_code as u8);
    for region_key in &summary.region_keys {
        write_region_key_digest(builder, *region_key);
    }
    for family_tag in &summary.family_tags {
        builder.write_u16(*family_tag);
    }
    if let Some(region_frame_ref) = summary.near_region_frame_ref {
        write_region_frame_ref_digest(builder, region_frame_ref);
    }
    for track_ref in &summary.far_phenomenon_track_refs {
        write_far_track_digest(builder, track_ref);
    }
}

fn write_region_key_digest(builder: &mut StableDigestBuilder, region_key: (i32, i32, i32)) {
    builder
        .write_u32(region_key.0 as u32)
        .write_u32(region_key.1 as u32)
        .write_u32(region_key.2 as u32);
}

fn write_region_frame_ref_digest(builder: &mut StableDigestBuilder, frame_ref: RegionFrameRef) {
    for component in frame_ref.geo_anchor_ref.coarse_world_origin_m {
        builder.write_u64(component as u64);
    }
    builder
        .write_u32(frame_ref.region_frame_ref.x as u32)
        .write_u32(frame_ref.region_frame_ref.y as u32)
        .write_u32(frame_ref.region_frame_ref.slab_z as u32);
    for component in frame_ref.region_origin_m {
        builder.write_u64(component as u64);
    }
}

fn write_far_track_digest(builder: &mut StableDigestBuilder, track_ref: &FarPhenomenonTrackRef) {
    builder
        .write_u64(track_ref.track_id)
        .write_u32(track_ref.anchor_region_frame_ref.x as u32)
        .write_u32(track_ref.anchor_region_frame_ref.y as u32)
        .write_u32(track_ref.anchor_region_frame_ref.slab_z as u32)
        .write_u16(track_ref.family_tag)
        .write_u8(track_ref.precision_zone_code as u8);
}
