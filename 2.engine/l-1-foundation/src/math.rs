//! Unified engine math backbone.

use crate::{EngineCoreError, EngineCoreResult, Invariant};
pub use glam::{Mat4 as Mat4f, Quat as Quatf, Vec2 as Vec2f, Vec3 as Vec3f, Vec4 as Vec4f};
use serde::{Deserialize, Serialize};

/// Axis-aligned bounds primitive used by higher layers.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Aabb3f {
    pub min: Vec3f,
    pub max: Vec3f,
}

impl Aabb3f {
    pub fn validate(&self) -> bool {
        self.min.cmple(self.max).all()
    }

    pub fn ensure_valid(&self) -> EngineCoreResult<()> {
        if self.validate() {
            Ok(())
        } else {
            Err(EngineCoreError::InvariantViolation(
                "aabb min must not exceed max",
            ))
        }
    }
}

impl Invariant for Aabb3f {
    fn check_invariants(&self) -> EngineCoreResult<()> {
        self.ensure_valid()
    }
}
