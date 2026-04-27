//! Authority Containers - Domain authority holders
//!
//! Per canon 02_STACK_MAP: L6.0 = Authority core (tooling).
//! Moved from 6.apps/editor per canonical boundary law.

pub mod audio;
pub mod material_authority_container;
pub mod terrain;

pub use audio::AudioAuthorityContainer;
pub use material_authority_container::MaterialAuthorityContainer;
pub use terrain::TerrainAuthorityContainer;
