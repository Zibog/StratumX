use super::state::WorldState;
use crate::{
    AppliedSegmentRecord, ApplySegment, WorldCausalSummary, MAX_FAMILY_FANOUT_PER_SEGMENT,
    MAX_PUBLISH_PASSES, MAX_SEGMENTS_PER_TICK,
};
use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_world_spatial::{
    publish_rebase_delta, CellFrameRef, FarPhenomenonTrackRef, PrecisionZoneCode, RegionAddress,
    RegionFrameRef, WorldCoordinate,
};
use std::collections::{BTreeMap, BTreeSet};

impl WorldState {
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

        let canonical_segments = canonicalize_segments(segments)?;
        let next_tick = Tick(self.tick.0.saturating_add(1));
        let next_epoch = self.epoch.saturating_add(1);
        let previous_region = self
            .active_region_frame_ref
            .map(|frame_ref| frame_ref.region_frame_ref);
        let anchor_region = canonical_segments.first().map(|segment| RegionAddress {
            x: segment.region_key.0,
            y: segment.region_key.1,
            slab_z: segment.region_key.2,
        });

        for segment in &canonical_segments {
            self.regions.register_region(
                RegionAddress {
                    x: segment.region_key.0,
                    y: segment.region_key.1,
                    slab_z: segment.region_key.2,
                },
                next_tick,
            );
        }

        let mut precision_zone_code = PrecisionZoneCode::ReducedExact;
        let mut far_phenomenon_track_refs = Vec::new();
        if let Some(anchor_region) = anchor_region {
            let near_region_frame_ref = RegionFrameRef::new(self.geo_anchor_ref, anchor_region);
            self.active_region_frame_ref = Some(near_region_frame_ref);
            self.active_cell_frame_ref = Some(CellFrameRef::new(anchor_region, [0, 0, 0]));
            self.last_rebase_delta_ref = match previous_region {
                Some(previous_region) if previous_region != anchor_region => Some(
                    publish_rebase_delta(previous_region, anchor_region).map_err(|_| {
                        EngineCoreError::InvalidDescriptor("world rebase delta publication failed")
                    })?,
                ),
                _ => None,
            };

            for segment in &canonical_segments {
                let segment_region = RegionAddress {
                    x: segment.region_key.0,
                    y: segment.region_key.1,
                    slab_z: segment.region_key.2,
                };
                let zone = precision_zone_for_region(segment_region, anchor_region);
                if precision_zone_rank(zone) > precision_zone_rank(precision_zone_code) {
                    precision_zone_code = zone;
                }
                if matches!(
                    zone,
                    PrecisionZoneCode::FarSummary | PrecisionZoneCode::TheaterSummary
                ) {
                    for family_tag in &segment.family_tags {
                        far_phenomenon_track_refs.push(
                            FarPhenomenonTrackRef::new(
                                stable_far_track_id(segment.region_key, *family_tag),
                                segment_region,
                                *family_tag,
                                zone,
                            )
                            .map_err(|_| {
                                EngineCoreError::InvalidDescriptor(
                                    "far phenomenon track publication failed",
                                )
                            })?,
                        );
                    }
                }
            }
        } else {
            self.last_rebase_delta_ref = None;
        }

        self.last_causal_summary = WorldCausalSummary {
            region_keys: canonical_segments
                .iter()
                .map(|segment| segment.region_key)
                .collect(),
            family_tags: canonical_segments
                .iter()
                .flat_map(|segment| segment.family_tags.iter().copied())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            publish_passes,
            near_region_frame_ref: self.active_region_frame_ref,
            far_phenomenon_track_refs,
            precision_zone_code,
        };
        self.last_apply_journal = crate::WorldApplyJournal {
            tick: next_tick,
            epoch: next_epoch,
            publish_passes,
            segment_count: canonical_segments.len(),
            segments: canonical_segments,
        };
        self.tick = next_tick;
        self.epoch = next_epoch;
        Ok(())
    }
}

fn canonicalize_segments(segments: &[ApplySegment]) -> EngineCoreResult<Vec<AppliedSegmentRecord>> {
    let mut by_region: BTreeMap<(i32, i32, i32), BTreeSet<u16>> = BTreeMap::new();
    for segment in segments {
        if segment.family_tags.is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "apply segment must contain at least one family tag",
            ));
        }
        let family_tags = by_region.entry(segment.region_key).or_default();
        family_tags.extend(segment.family_tags.iter().copied());
    }

    Ok(by_region
        .into_iter()
        .map(|(region_key, family_tags)| AppliedSegmentRecord {
            region_key,
            family_tags: family_tags.into_iter().collect(),
        })
        .collect())
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    bytes.iter().fold(OFFSET, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(PRIME)
    })
}

fn precision_zone_for_region(
    region: RegionAddress,
    anchor_region: RegionAddress,
) -> PrecisionZoneCode {
    let region_origin = engine_world_spatial::region_origin_meters(region);
    let coord = WorldCoordinate {
        meters: region_origin,
    };
    engine_world_spatial::precision_zone_for_coordinate(coord, anchor_region)
}

fn precision_zone_rank(zone: PrecisionZoneCode) -> u8 {
    match zone {
        PrecisionZoneCode::LocalContact => 0,
        PrecisionZoneCode::Interactive => 1,
        PrecisionZoneCode::ReducedExact => 2,
        PrecisionZoneCode::FarSummary => 3,
        PrecisionZoneCode::TheaterSummary => 4,
    }
}

fn stable_far_track_id(region_key: (i32, i32, i32), family_tag: u16) -> u64 {
    let mut bytes = Vec::with_capacity(14);
    bytes.extend_from_slice(&region_key.0.to_le_bytes());
    bytes.extend_from_slice(&region_key.1.to_le_bytes());
    bytes.extend_from_slice(&region_key.2.to_le_bytes());
    bytes.extend_from_slice(&family_tag.to_le_bytes());
    fnv1a64(&bytes).max(1)
}
