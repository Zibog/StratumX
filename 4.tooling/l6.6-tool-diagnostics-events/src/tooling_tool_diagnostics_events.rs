pub mod diagnostics_publisher;
pub use diagnostics_publisher::*;

pub const CANONICAL_LEVEL: &str = "l6.6-tool-diagnostics-events";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct L66ToolDiagnosticsEventsMarker;
