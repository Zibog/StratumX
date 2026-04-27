use super::types::{IkJoint, IkTarget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IkChain {
    pub joints: Vec<IkJoint>,
    pub end_effector_index: usize,
}

impl IkChain {
    pub fn new() -> Self {
        Self {
            joints: Vec::new(),
            end_effector_index: 0,
        }
    }

    pub fn add_joint(
        &mut self,
        position: [f32; 3],
        parent_index: Option<usize>,
        length: f32,
    ) -> usize {
        let joint = IkJoint {
            position,
            rotation: [0.0, 0.0, 0.0, 1.0],
            parent_index,
            length,
        };
        self.joints.push(joint);
        self.joints.len() - 1
    }

    pub fn set_end_effector(&mut self, index: usize) {
        self.end_effector_index = index;
    }

    pub fn solve_reach(&mut self, target: [f32; 3], iterations: usize, tolerance: f32) -> IkTarget {
        if self.joints.is_empty() {
            return IkTarget {
                position: target,
                reached: false,
                distance_to_target: f32::MAX,
            };
        }

        let root_position = self.joints[0].position;

        for _ in 0..iterations {
            if let Some(end_joint) = self.joints.get_mut(self.end_effector_index) {
                end_joint.position = target;
            }

            for i in (1..self.joints.len()).rev() {
                if let Some(parent_idx) = self.joints[i].parent_index {
                    let child_pos = self.joints[i].position;
                    let parent_pos = self.joints[parent_idx].position;
                    let length = self.joints[i].length;

                    let dx = child_pos[0] - parent_pos[0];
                    let dy = child_pos[1] - parent_pos[1];
                    let dz = child_pos[2] - parent_pos[2];
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt();

                    if dist > 0.001 {
                        let ratio = length / dist;
                        self.joints[i].position = [
                            parent_pos[0] + dx * ratio,
                            parent_pos[1] + dy * ratio,
                            parent_pos[2] + dz * ratio,
                        ];
                    }
                }
            }

            self.joints[0].position = root_position;

            for i in 1..self.joints.len() {
                if let Some(parent_idx) = self.joints[i].parent_index {
                    let child_pos = self.joints[i].position;
                    let parent_pos = self.joints[parent_idx].position;
                    let length = self.joints[i].length;

                    let dx = child_pos[0] - parent_pos[0];
                    let dy = child_pos[1] - parent_pos[1];
                    let dz = child_pos[2] - parent_pos[2];
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt();

                    if dist > 0.001 {
                        let ratio = length / dist;
                        self.joints[i].position = [
                            parent_pos[0] + dx * ratio,
                            parent_pos[1] + dy * ratio,
                            parent_pos[2] + dz * ratio,
                        ];
                    }
                }
            }

            let end_pos = self.joints[self.end_effector_index].position;
            let dx = target[0] - end_pos[0];
            let dy = target[1] - end_pos[1];
            let dz = target[2] - end_pos[2];
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            if distance < tolerance {
                return IkTarget {
                    position: end_pos,
                    reached: true,
                    distance_to_target: distance,
                };
            }
        }

        let end_pos = self.joints[self.end_effector_index].position;
        let dx = target[0] - end_pos[0];
        let dy = target[1] - end_pos[1];
        let dz = target[2] - end_pos[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        IkTarget {
            position: end_pos,
            reached: distance < tolerance,
            distance_to_target: distance,
        }
    }
}

impl Default for IkChain {
    fn default() -> Self {
        Self::new()
    }
}
