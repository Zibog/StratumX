//! Shared command gate error types.
//!
//! This module centralizes the rejection payload types reused by every command
//! gate split out of the legacy monolith.

pub(crate) use sdk_compat::verdicts::{LegalityRejection, LegalityRejectionReason};
