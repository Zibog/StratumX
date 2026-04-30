use std::collections::BTreeSet;

use crate::ApplyPayload;

pub fn has_duplicate_operations(payload: &ApplyPayload) -> bool {
    duplicate_structural(payload) || duplicate_writes(payload)
}

pub fn has_conflicting_operations(payload: &ApplyPayload) -> bool {
    let structural_set: BTreeSet<_> = payload.change_set.structural.iter().copied().collect();
    payload
        .change_set
        .writes
        .iter()
        .any(|write| structural_set.contains(&write.component))
}

fn duplicate_structural(payload: &ApplyPayload) -> bool {
    let mut seen = BTreeSet::new();
    payload
        .change_set
        .structural
        .iter()
        .any(|component| !seen.insert(*component))
}

fn duplicate_writes(payload: &ApplyPayload) -> bool {
    let mut seen = BTreeSet::new();
    payload
        .change_set
        .writes
        .iter()
        .any(|write| !seen.insert(write.component))
}
