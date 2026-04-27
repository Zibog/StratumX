//! l2-critical-simulation-families/kinetics: Physics and ballistics simulation
//!
//! Provides physics simulation, ballistic trajectories, collision response,
//! and kinetic energy modeling for game world interactions.

mod types;
mod runtime;
mod validation;
mod queries;
mod exports;

pub use exports::*;
