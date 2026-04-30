//! World Session Service - Persistence
//!
//! Persistence currently flows through world state and project state.
//! This module preserves the persistence split point for later expansion.

// This module is intentionally minimal as persistence operations are currently
// handled through the WorldState's world_snapshot_ref field and ProjectState's save_generation.
// Later persistence-specific operations can be added here without regrowing session.rs.
