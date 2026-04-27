// Audio occlusion - sound blocking

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Occluder {
    pub position: [f32; 3],
    pub bounds: [f32; 3],
    pub occlusion_factor: f32,
}

impl Occluder {
    pub fn occludes(&self, source_pos: [f32; 3], listener_pos: [f32; 3]) -> f32 {
        let source_to_listener_dist = ((listener_pos[0] - source_pos[0]).powi(2)
            + (listener_pos[1] - source_pos[1]).powi(2)
            + (listener_pos[2] - source_pos[2]).powi(2))
        .sqrt();

        let source_to_occluder_dist = ((self.position[0] - source_pos[0]).powi(2)
            + (self.position[1] - source_pos[1]).powi(2)
            + (self.position[2] - source_pos[2]).powi(2))
        .sqrt();

        if source_to_occluder_dist >= source_to_listener_dist || source_to_occluder_dist < 0.1 {
            return 0.0;
        }

        let t = source_to_occluder_dist / source_to_listener_dist;
        let point_on_line = [
            source_pos[0] + (listener_pos[0] - source_pos[0]) * t,
            source_pos[1] + (listener_pos[1] - source_pos[1]) * t,
            source_pos[2] + (listener_pos[2] - source_pos[2]) * t,
        ];

        let dx = (point_on_line[0] - self.position[0]).abs();
        let dy = (point_on_line[1] - self.position[1]).abs();
        let dz = (point_on_line[2] - self.position[2]).abs();

        if dx < self.bounds[0] && dy < self.bounds[1] && dz < self.bounds[2] {
            self.occlusion_factor
        } else {
            0.0
        }
    }
}
