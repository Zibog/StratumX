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
impl ObjectHandle {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectClass {
    World,
    Scene,
    Terrain,
    Material,
    Logic,
    Asset,
    Build,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolingError {
    UnknownObject,
    Message(String),
}
impl From<String> for ToolingError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    next_handle: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationAiAuthoringSuite {
    pub logic_nodes: Vec<ObjectHandle>,
}
impl EditorProduct {
    pub fn create_object_with_class(
        &mut self,
        _label: impl Into<String>,
        _class: ObjectClass,
    ) -> Result<ObjectHandle, ToolingError> {
        self.next_handle += 1;
        Ok(ObjectHandle::new(self.next_handle))
    }

    pub fn create_logic_node(
        &mut self,
        label: impl Into<String>,
    ) -> Result<ObjectHandle, ToolingError> {
        self.create_object_with_class(label, ObjectClass::Logic)
    }
}
