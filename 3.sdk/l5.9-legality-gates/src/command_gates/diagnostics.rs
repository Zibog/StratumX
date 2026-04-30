//! Diagnostics command legality gates.
//!
//! This module contains command checks for operator-facing inspection and
//! causal explanation flows.

use super::common::{non_zero_u32, GateResult};
use super::verdict::LegalityVerdict;

pub fn validate_reason_chain_inspect_npc(npc_id: u32) -> GateResult {
    non_zero_u32(npc_id, "npc_id", "NPC ID")?;
    Ok(LegalityVerdict::Legal)
}
