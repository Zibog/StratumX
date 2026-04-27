//! Keyframe animation system

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Keyframe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub id: Uuid,
    pub time: f32,
    pub value: KeyframeValue,
    pub interpolation: InterpolationType,
}

/// Keyframe value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyframeValue {
    Float(f32),
    Vec3([f32; 3]),
    Quaternion([f32; 4]),
    Bool(bool),
    Event(String),
}

/// Interpolation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    Step,
    Bezier,
    Smooth,
}

impl Keyframe {
    pub fn new(time: f32, value: KeyframeValue) -> Self {
        Self {
            id: Uuid::new_v4(),
            time,
            value,
            interpolation: InterpolationType::Linear,
        }
    }

    pub fn with_interpolation(mut self, interpolation: InterpolationType) -> Self {
        self.interpolation = interpolation;
        self
    }
}

/// Keyframe operations
pub struct KeyframeOps;

impl KeyframeOps {
    pub fn interpolate(k1: &Keyframe, k2: &Keyframe, t: f32) -> Option<KeyframeValue> {
        if t < k1.time || t > k2.time {
            return None;
        }

        let alpha = (t - k1.time) / (k2.time - k1.time);

        match (&k1.value, &k2.value) {
            (KeyframeValue::Float(v1), KeyframeValue::Float(v2)) => {
                Some(KeyframeValue::Float(Self::lerp_f32(*v1, *v2, alpha)))
            }
            (KeyframeValue::Vec3(v1), KeyframeValue::Vec3(v2)) => {
                Some(KeyframeValue::Vec3(Self::lerp_vec3(*v1, *v2, alpha)))
            }
            _ => None,
        }
    }

    fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    fn lerp_vec3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
        [
            Self::lerp_f32(a[0], b[0], t),
            Self::lerp_f32(a[1], b[1], t),
            Self::lerp_f32(a[2], b[2], t),
        ]
    }
}
