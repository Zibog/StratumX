pub use authoring_runtime::*;
pub use vertical_slice_runtime::*;

mod authoring_runtime;
mod vertical_slice_runtime;

#[cfg(feature = "desktop")]
pub mod app_host;

pub const CANONICAL_LEVEL: &str = "l6.12-preview-runtime";

// REAL RUNTIME PATH: Helper to wire vertical slice session into command executor
pub fn initialize_vertical_slice_in_executor(
    executor: &mut stratumx_tooling_l6_0_tool_session::CommandExecutor,
) -> Result<(), String> {
    let session = VerticalSliceSession::new()?;
    executor
        .packet_executor_mut()
        .set_vertical_slice_session(Box::new(session));
    Ok(())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L612PreviewRuntimeMarker;
