//! Animation runtime and keyframe control
//! **Owner**: engine_animation -- Keyframe-based animation synthesis

pub mod animation_runtime;
mod types;
mod runtime;
mod validation;
mod queries;
mod exports;

pub use exports::*;
