//! Audio Registry - preview operations.
//!
//! Contains audibility, obstruction, transition, and voice subtitle preview functions.

use crate::model::{
    AudibilityPreview, AudioRegistry, ObjectHandle, ObstructionOcclusionResult, TransitionResult,
    VoiceSubtitleLegality,
};

impl AudioRegistry {
    pub fn preview_audibility(
        &self,
        listener_position: [f32; 3],
        max_distance: f32,
    ) -> AudibilityPreview {
        let listener_pos = [
            listener_position[0] as i32,
            listener_position[1] as i32,
            listener_position[2] as i32,
        ];
        let max_dist_sq = (max_distance * max_distance) as i32;

        let mut audible = Vec::new();
        let mut occluded = Vec::new();
        let mut attenuated = Vec::new();

        for (handle, source) in &self.sources {
            let dx = source.position[0] - listener_pos[0];
            let dy = source.position[1] - listener_pos[1];
            let dz = source.position[2] - listener_pos[2];
            let dist_sq = dx * dx + dy * dy + dz * dz;

            if dist_sq < max_dist_sq {
                audible.push(*handle);
                let attenuation = ((dist_sq as f32 / max_dist_sq as f32) * 100.0) as u32;
                attenuated.push((*handle, attenuation));
            } else {
                occluded.push(*handle);
            }
        }

        AudibilityPreview {
            listener_position: listener_pos,
            audible_sources: audible,
            occluded_sources: occluded,
            distance_attenuated: attenuated,
        }
    }

    pub fn preview_obstruction_occlusion(
        &self,
        source: ObjectHandle,
        listener_position: [f32; 3],
    ) -> Result<ObstructionOcclusionResult, String> {
        let _source_data = self.sources.get(&source).ok_or("Source not found")?;

        let listener_pos = [
            listener_position[0] as i32,
            listener_position[1] as i32,
            listener_position[2] as i32,
        ];

        let obstructed = false;
        let occluded = false;

        Ok(ObstructionOcclusionResult {
            source,
            listener_position: listener_pos,
            obstructed,
            occluded,
            obstruction_factor: 0,
            occlusion_factor: 0,
        })
    }

    pub fn preview_transition(
        &self,
        from_zone: ObjectHandle,
        to_zone: ObjectHandle,
    ) -> Result<TransitionResult, String> {
        let _from = self.zones.get(&from_zone).ok_or("From zone not found")?;
        let _to = self.zones.get(&to_zone).ok_or("To zone not found")?;

        let transition_valid = true;
        let blend_curve = vec![0, 25, 50, 75, 100];

        Ok(TransitionResult {
            from_zone,
            to_zone,
            transition_valid,
            reverb_blend_curve: blend_curve,
        })
    }

    pub fn preview_voice_subtitle_legality(&self, dialogue_id: String) -> VoiceSubtitleLegality {
        VoiceSubtitleLegality {
            dialogue_id,
            voice_present: true,
            subtitle_present: true,
            locale_legal: true,
            missing_locales: Vec::new(),
        }
    }
}
