use super::ik_solver::IkChain;
use super::types::{
    AnimationClip, AnimationFailure, AnimationFailureReason, AnimationState, IkTarget,
};
use engine_core::{EngineCoreResult, StableDigestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationRuntime {
    pub clips: Vec<AnimationClip>,
    pub states: Vec<AnimationState>,
    pub ik_chains: Vec<IkChain>,
}

impl AnimationRuntime {
    pub fn new() -> Self {
        Self {
            clips: Vec::new(),
            states: Vec::new(),
            ik_chains: Vec::new(),
        }
    }

    pub fn add_clip(&mut self, clip: AnimationClip) -> usize {
        self.clips.push(clip);
        self.clips.len() - 1
    }

    pub fn play_clip(&mut self, clip_name: &str) -> bool {
        if self.clips.iter().any(|c| c.name == clip_name) {
            self.states.push(AnimationState {
                clip_name: clip_name.to_string(),
                current_time: 0.0,
                playing: true,
                speed: 1.0,
            });
            true
        } else {
            false
        }
    }

    pub fn add_ik_chain(&mut self, chain: IkChain) -> usize {
        self.ik_chains.push(chain);
        self.ik_chains.len() - 1
    }

    pub fn solve_ik(&mut self, chain_index: usize, target: [f32; 3]) -> Option<IkTarget> {
        self.ik_chains
            .get_mut(chain_index)
            .map(|chain| chain.solve_reach(target, 10, 0.01))
    }

    pub fn update(&mut self, delta_time: f32) {
        for state in &mut self.states {
            if state.playing {
                state.current_time += delta_time * state.speed;

                if let Some(clip) = self.clips.iter().find(|c| c.name == state.clip_name) {
                    if !clip.looping && state.current_time >= clip.duration_sec {
                        state.playing = false;
                    }
                }
            }
        }
    }
}

impl Default for AnimationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

use super::types::{AnimationReceipt, AnimationTier};

impl AnimationRuntime {
    /// Sample animation with receipt.
    pub fn sample_with_receipt(
        &self,
        clip_name: &str,
        sample_time: f32,
        joint_count: usize,
    ) -> EngineCoreResult<AnimationReceipt> {
        let clip = self
            .clips
            .iter()
            .find(|c| c.name == clip_name)
            .ok_or(AnimationFailure::new(
                AnimationFailureReason::MissingClip,
                "animation sample requires registered clip",
            ))?;

        if joint_count == 0 {
            return Err(AnimationFailure::new(
                AnimationFailureReason::MissingSkeleton,
                "animation sample requires non-zero joint count",
            )
            .into());
        }
        if clip.keyframes.is_empty() {
            return Err(AnimationFailure::new(
                AnimationFailureReason::InvalidSampleTime,
                "animation clip requires at least one keyframe",
            )
            .into());
        }
        if clip.duration_sec <= 0.0 {
            return Err(AnimationFailure::new(
                AnimationFailureReason::InvalidSampleTime,
                "animation clip requires positive duration",
            )
            .into());
        }
        if sample_time < 0.0 || sample_time > clip.duration_sec {
            return Err(AnimationFailure::new(
                AnimationFailureReason::InvalidSampleTime,
                "animation sample time lies outside clip duration",
            )
            .into());
        }

        let mut previous_time = None;
        let mut has_leading_keyframe = false;
        let mut has_trailing_keyframe = false;
        for keyframe in &clip.keyframes {
            if keyframe.time_sec < 0.0 || keyframe.time_sec > clip.duration_sec {
                return Err(AnimationFailure::new(
                    AnimationFailureReason::InvalidSampleTime,
                    "animation clip keyframes violate clip duration",
                )
                .into());
            }
            if let Some(previous_time) = previous_time {
                if keyframe.time_sec < previous_time {
                    return Err(AnimationFailure::new(
                        AnimationFailureReason::InvalidSampleTime,
                        "animation clip keyframes must be sorted by time",
                    )
                    .into());
                }
            }
            previous_time = Some(keyframe.time_sec);
            if keyframe.time_sec <= sample_time {
                has_leading_keyframe = true;
            }
            if keyframe.time_sec >= sample_time {
                has_trailing_keyframe = true;
            }
        }
        if !has_leading_keyframe || !has_trailing_keyframe {
            return Err(AnimationFailure::new(
                AnimationFailureReason::InvalidSampleTime,
                "animation sample time lacks enclosing keyframe span",
            )
            .into());
        }

        let selected_tier = if joint_count > 100 {
            AnimationTier::RootMotionOnly
        } else if joint_count > 50 {
            AnimationTier::ReducedPose
        } else {
            AnimationTier::FullPose
        };

        let sampled_frame = (sample_time * 60.0) as u64;

        let mut digest = StableDigestBuilder::new();
        digest
            .write_bytes(b"engine.animation.sample")
            .write_bytes(clip_name.as_bytes())
            .write_u32(sample_time.to_bits())
            .write_u64(sampled_frame)
            .write_u64(joint_count as u64)
            .write_u8(selected_tier as u8);

        Ok(AnimationReceipt {
            selected_tier,
            sampled_frame,
            deterministic_digest: digest.finish().0,
        })
    }
}
