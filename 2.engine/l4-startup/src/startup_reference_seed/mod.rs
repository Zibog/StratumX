// Startup Reference Seed - seed data composition

pub mod bindings;
pub mod bootstrap;
pub mod environment_seed;
pub mod inventory_seed;
pub mod materials_seed;
pub mod seed_ids;
pub mod streaming_seed;
pub mod structures_seed;
pub mod terrain_seed;
pub mod world_identity;

pub use bootstrap::{launch_startup_reference_seed, StartupReferenceSeedRuntime};
