//! FUTURE_STUB.
//!
//! This crate is present as a canonical future surface.
//! It is not part of the active product spine yet.
//! It must not be counted as product-complete.

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectHandle(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationCinematicsAuthoringSuite {
    pub timelines: Vec<ObjectHandle>,
}
