use super::types::{AnimationClip, Keyframe};

impl AnimationClip {
    pub fn sample(&self, time_sec: f32, joint_index: usize) -> Option<([f32; 3], [f32; 4])> {
        let mut time = time_sec;
        if self.looping {
            time %= self.duration_sec;
        } else {
            time = time.min(self.duration_sec);
        }

        let mut prev_kf: Option<&Keyframe> = None;
        let mut next_kf: Option<&Keyframe> = None;

        for kf in &self.keyframes {
            if kf.joint_index != joint_index {
                continue;
            }

            if kf.time_sec <= time {
                prev_kf = Some(kf);
            }
            if kf.time_sec >= time && next_kf.is_none() {
                next_kf = Some(kf);
            }
        }

        match (prev_kf, next_kf) {
            (Some(prev), Some(next)) if prev.time_sec != next.time_sec => {
                let t = (time - prev.time_sec) / (next.time_sec - prev.time_sec);
                let position = [
                    prev.position[0] + (next.position[0] - prev.position[0]) * t,
                    prev.position[1] + (next.position[1] - prev.position[1]) * t,
                    prev.position[2] + (next.position[2] - prev.position[2]) * t,
                ];
                let rotation = [
                    prev.rotation[0] + (next.rotation[0] - prev.rotation[0]) * t,
                    prev.rotation[1] + (next.rotation[1] - prev.rotation[1]) * t,
                    prev.rotation[2] + (next.rotation[2] - prev.rotation[2]) * t,
                    prev.rotation[3] + (next.rotation[3] - prev.rotation[3]) * t,
                ];
                Some((position, rotation))
            }
            (Some(kf), _) | (_, Some(kf)) => Some((kf.position, kf.rotation)),
            _ => None,
        }
    }
}
