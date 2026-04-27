// Ballistic Simulation - modular structure

pub mod impact;
pub mod integration;
pub mod penetration;
pub mod projectile;
pub mod verdicts;

pub use projectile::BallisticSimulator;
