//! FUTURE_STUB: this crate is intentionally not product-integrated yet.
//! It must not be counted as implemented editor functionality until wired into the active product spine.

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionRoutingSystem {
    pub command_palette_open: bool,
    pub pointer_captured: bool,
}
