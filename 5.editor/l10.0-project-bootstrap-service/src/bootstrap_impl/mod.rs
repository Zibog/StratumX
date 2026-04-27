//! Editor Project Bootstrap Service

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling::{
    ApprovalClass, BudgetClass, BuildArtifact, CommandOrigin, ObjectClass, ObjectHandle,
    ReleasePackage, ToolCommand, ToolObject, ToolSnapshot, ToolingError, ToolingRuntime,
};
pub use stratumx_tooling_l6_0_tool_session as stratumx_tooling;

mod product;
mod types;

pub use product::EditorProduct;
pub use types::*;

mod bootstrap_types;
mod export_document_runtime;
mod import_document_runtime;
mod persistence_runtime;
mod reference_bootstrap_runtime;
mod reference_shell_runtime;

#[cfg(feature = "desktop")]
pub mod desktop;

pub use bootstrap_types::*;
