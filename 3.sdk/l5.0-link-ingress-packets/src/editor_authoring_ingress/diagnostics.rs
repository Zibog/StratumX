//! Diagnostic command types module
//!
//! Narrow re-export module for controlled transition after monolith split.

pub mod types;

pub use types::{
    DestructionCommand, EcologyCommand, MaterialArchetypeInput, MaterialCommand,
    MaterialLayerInput, MaterialWorldCommand, NavDoorInventoryCommand, PopulationCommand,
    ReasonChainCommand, TacticsCommand,
};
