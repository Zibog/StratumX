# Boundary Preservation

This contract belongs specifically to the l6a.f1 assistant safety family and describes family-only coordination.


## Must remain outside `assistant_safety_family` ownership
- member-internal mutable authority rows for assistant safety, approval, legality, and apply/revert policy composition, authority-facing minimal truth: safety/approval refs only, snapshot classes: safety snapshots, index classes: safety lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6a.f1-assistant-safety-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
