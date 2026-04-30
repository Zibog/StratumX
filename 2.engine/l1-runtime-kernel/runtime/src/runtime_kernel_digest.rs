use crate::RuntimeKernel;
use engine_core::{EngineCoreResult, StableDigestBuilder};
use engine_world::ApplySegment;

impl RuntimeKernel {
    pub fn deterministic_digest(&self) -> EngineCoreResult<u64> {
        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.runtime.kernel")
            .write_u8(self.config.profile as u8)
            .write_u8(self.phase as u8)
            .write_u64(self.apply_queue.len() as u64)
            .write_u64(self.transfer_completion_queue.len() as u64)
            .write_u64(self.connection_publication_queue.len() as u64)
            .write_u64(self.presentable_frame_queue.len() as u64);
        for segment in &self.apply_queue {
            write_apply_segment_digest(&mut digest, segment);
        }
        for completion in &self.transfer_completion_queue {
            digest
                .write_u64(completion.transfer_id)
                .write_u64(completion.bytes as u64);
        }
        for (connection, records) in &self.connection_publication_queue {
            digest
                .write_u64(connection.0)
                .write_u64(records.len() as u64);
            for record in records {
                digest.write_u64(record.order).write_bytes(&record.bytes);
            }
        }
        for frame in &self.presentable_frame_queue {
            digest
                .write_u64(frame.frame_id)
                .write_u8(frame.visibility_freshness_frames);
        }
        digest.write_u64(self.world.deterministic_digest()?);
        Ok(digest.finish().0)
    }
}

fn write_apply_segment_digest(builder: &mut StableDigestBuilder, segment: &ApplySegment) {
    builder
        .write_u32(segment.region_key.0 as u32)
        .write_u32(segment.region_key.1 as u32)
        .write_u32(segment.region_key.2 as u32)
        .write_u64(segment.family_tags.len() as u64);
    for family_tag in &segment.family_tags {
        builder.write_u16(*family_tag);
    }
}
