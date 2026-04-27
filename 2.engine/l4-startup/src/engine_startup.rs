#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]

pub use asset_root::*;
pub use sky_bundle::*;
pub use startup_assembly::*;
pub use startup_types::*;

pub mod asset_root;
pub mod sky_bundle;
mod startup_assembly;
pub mod startup_reference_seed;
mod startup_types;

pub use startup_reference_seed::{
    launch_startup_reference_seed, StartupReferenceSeedRuntime, REFERENCE_SCENE_NAME,
};
