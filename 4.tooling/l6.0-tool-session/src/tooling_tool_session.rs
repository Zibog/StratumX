// Tooling Tool Session

// Domain executor directories
pub mod api;
pub mod audio;
pub mod build;
pub mod common;
pub mod diagnostics;
pub mod diagnostics_integration;
pub mod environment;
pub mod executors;
pub mod material;
pub mod project;
pub mod recovery_integration;
pub mod runtime;
pub mod terrain;
pub mod world;

// Routing and session layers
pub mod routing;
pub mod session;

// Internal tool-session modules
mod tooling_tool_session {
    pub mod audio_executor;
    pub mod build_release;
    pub mod commands;
    pub mod material_executor;
    pub mod preview_planning;
    pub mod runtime;
    pub mod types;

    // Split module implementations
    pub(crate) mod runtime_core_impl;
    pub(crate) mod runtime_ops;
    pub(crate) mod types_core;
    pub(crate) mod types_session;
}

pub use tooling_tool_session::commands::{ToolCommand, ToolCommandResult};
pub use tooling_tool_session::runtime::ToolingRuntime;
pub use tooling_tool_session::types::*;

// Re-export DisabledReason for executor use
pub use api::command_executor::CanonicalCommandExecutor;
pub use common::context::ToolSessionContext;
pub use common::publication::EditorPublication;
pub use common::result::{
    ArtifactRef, CommandResult, CommandVerdict, DiagnosticsRecord, LifecycleState,
};
pub use tooling_tool_session::types::DisabledReason;

// Legacy PromotedCommand-based executor — kept for spine_core.rs compatibility.
// New code should use CanonicalCommandExecutor via the api/ layer.
pub mod command_executor;
pub use command_executor::CommandExecutor;

// Re-export domain executors for integration tests
pub use audio::executor_audio::AudioExecutor;
pub use build::executor_build::BuildExecutor;
pub use common::executor_shell::ShellExecutor;
pub use environment::executor_environment::EnvironmentExecutor;
pub use material::executor_material::MaterialExecutor;
pub use runtime::executor_runtime::RuntimeExecutor;
pub use terrain::executor_terrain::TerrainExecutor;
pub use world::executor_world::WorldExecutor;
