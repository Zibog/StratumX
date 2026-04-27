use super::ik_solver::IkChain;
use super::types::{AnimationClip, AnimationState, IkTarget};
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
