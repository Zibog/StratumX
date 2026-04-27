# Boundary Preservation

This contract belongs specifically to the l6.f0 workspace shell family and describes family-only coordination.


## Must remain outside `workspace_shell_family` ownership
- member-internal mutable authority rows for workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing, authority-facing minimal truth: workspace session ownership only, snapshot classes: workspace snapshots and session-scoped view snapshots, index classes: workspace/session lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f15-workspace-shell-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
