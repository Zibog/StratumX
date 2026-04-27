mod creature_profile;
mod ecology_state;
mod migration;

pub use creature_profile::{CreatureProfile, CreatureSpecies, MigrationReason};
pub use ecology_state::CreatureEcologyState;
pub use migration::{MigrationCorridor, MigrationState};
