//! Shared legality gate helpers.
//!
//! This module keeps the legacy monolith's small validation primitives and
//! rejection construction out of the domain-specific gate modules.

use super::errors::{LegalityRejection, LegalityRejectionReason};
use super::verdict::LegalityVerdict;

pub(crate) type GateResult = Result<LegalityVerdict, LegalityRejection>;

pub(crate) fn invalid(field: &str, message: impl Into<String>) -> LegalityRejection {
    LegalityRejection {
        verdict: LegalityVerdict::Illegal,
        reason: LegalityRejectionReason::InvalidInput {
            field: field.into(),
            message: message.into(),
        },
    }
}

pub(crate) fn non_zero_u32(value: u32, field: &str, label: &str) -> Result<(), LegalityRejection> {
    if value == 0 {
        Err(invalid(field, format!("{label} cannot be zero")))
    } else {
        Ok(())
    }
}

pub(crate) fn non_zero_u16(value: u16, field: &str, label: &str) -> Result<(), LegalityRejection> {
    if value == 0 {
        Err(invalid(field, format!("{label} cannot be zero")))
    } else {
        Ok(())
    }
}

pub(crate) fn non_zero_usize(
    value: usize,
    field: &str,
    label: &str,
) -> Result<(), LegalityRejection> {
    if value == 0 {
        Err(invalid(field, format!("{label} cannot be zero")))
    } else {
        Ok(())
    }
}

pub(crate) fn non_empty(value: &str, field: &str, message: &str) -> Result<(), LegalityRejection> {
    if value.is_empty() {
        Err(invalid(field, message))
    } else {
        Ok(())
    }
}

pub(crate) fn max_len(
    value: &str,
    max: usize,
    field: &str,
    label: &str,
) -> Result<(), LegalityRejection> {
    if value.len() > max {
        Err(invalid(
            field,
            format!("{label} too long: {} > {max}", value.len()),
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn positive(value: f32, field: &str, label: &str) -> Result<(), LegalityRejection> {
    if value <= 0.0 {
        Err(invalid(
            field,
            format!("{label} must be positive, got {value}"),
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn non_negative(value: f32, field: &str, label: &str) -> Result<(), LegalityRejection> {
    if value < 0.0 {
        Err(invalid(
            field,
            format!("{label} cannot be negative, got {value}"),
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn normalized(value: f32, field: &str, label: &str) -> Result<(), LegalityRejection> {
    if !(0.0..=1.0).contains(&value) {
        Err(invalid(field, format!("{label} must be 0-1, got {value}")))
    } else {
        Ok(())
    }
}
