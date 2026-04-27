//! Action Queries - State Query Interface
//!
//! Provides the StateQueries trait for read-only access to editor state
//! during action dispatch validation.
//!
//! ## Canonical Architecture:
//! ```text
//! action_queries.rs (this) - StateQueries trait
//!         ↓
//! action_validation.rs - uses StateQueries
//!         ↓
//! action_dispatch.rs - orchestrates validation + execution
//! ```

/// State query interface for read-only access to editor state.
///
/// This trait provides the ActionDispatch with access to state needed
/// for context validation without allowing mutation.
pub trait StateQueries: Send + Sync {
    /// Returns true if a project is currently open
    fn has_project(&self) -> bool;

    /// Returns true if a world is currently open
    fn has_active_world(&self) -> bool;

    /// Returns true if any entities are selected
    fn has_selection(&self) -> bool;

    /// Returns true if a transaction is currently active
    fn is_transaction_active(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockQueries {
        project: bool,
        world: bool,
        selection: bool,
        transaction: bool,
    }

    impl StateQueries for MockQueries {
        fn has_project(&self) -> bool {
            self.project
        }
        fn has_active_world(&self) -> bool {
            self.world
        }
        fn has_selection(&self) -> bool {
            self.selection
        }
        fn is_transaction_active(&self) -> bool {
            self.transaction
        }
    }

    #[test]
    fn test_state_queries_trait_object() {
        let queries: Arc<dyn StateQueries> = Arc::new(MockQueries {
            project: true,
            world: false,
            selection: true,
            transaction: false,
        });

        assert!(queries.has_project());
        assert!(!queries.has_active_world());
        assert!(queries.has_selection());
        assert!(!queries.is_transaction_active());
    }
}
