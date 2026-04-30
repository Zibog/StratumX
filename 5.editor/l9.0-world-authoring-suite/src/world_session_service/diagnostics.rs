//! World Session Service - Diagnostics
//!
//! Diagnostics remain owned by world state in the current live contour.
//! This module preserves the diagnostics split point for later expansion.

// This module is intentionally minimal as diagnostics are currently
// handled through the WorldState's world_diagnostics field.
// Later diagnostics-specific operations can be added here without regrowing session.rs.
