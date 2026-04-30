use serde::{Deserialize, Serialize};

mod destruction;
mod ecology;
mod material;
mod nav_door_inventory;
mod population;
mod reason_chain;
mod tactics;

pub use destruction::DestructionCommand;
pub use ecology::EcologyCommand;
pub use material::{
    MaterialArchetypeInput, MaterialCommand, MaterialLayerInput, MaterialWorldCommand,
};
pub use nav_door_inventory::NavDoorInventoryCommand;
pub use population::PopulationCommand;
pub use reason_chain::ReasonChainCommand;
pub use tactics::TacticsCommand;
