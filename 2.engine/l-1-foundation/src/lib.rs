//! Canonical L-1 foundation crate for `StratumX`.

pub mod contracts;
pub mod digest;
pub mod error;
pub mod flags;
pub mod math;
pub mod types;

pub use contracts::{EngineDescriptor, Invariant};
pub use digest::{StableDigest64, StableDigestBuilder};
pub use error::{EngineCoreError, EngineCoreResult};
pub use flags::{FeatureProfile, ACTIVE_PROFILE};
pub use math::{Aabb3f, Mat4f, Quatf, Vec2f, Vec3f, Vec4f};
pub use types::{ComponentTypeId, Generation, Tick};
