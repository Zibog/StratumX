//! Shared command gate verdict types.
//!
//! This module keeps the command-gate split anchored to the existing SDK
//! verdict model without re-embedding those definitions into every file.

pub(crate) use sdk_compat::verdicts::LegalityVerdict;
