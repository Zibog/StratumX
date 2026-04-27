//! Editor context module - coordinator, services, state, session, and state management.

pub mod editor_coordinator;
pub mod editor_services;
pub mod editor_state;
pub mod session;
pub mod state_container_management;
pub mod state_queries;

// Re-exports for convenience
pub use editor_coordinator::{EditorCoordinator, Severity};
pub use editor_services::EditorServices;
pub use editor_state::EditorState;
pub use session::{EditorSession, RuntimeMode};
pub use state_queries::{SessionTerrainQueries, StateQueries, TerrainDataSource};
