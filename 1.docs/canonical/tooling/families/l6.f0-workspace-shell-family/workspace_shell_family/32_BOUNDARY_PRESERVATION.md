# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f0 workspace shell family family and may not be reused by a different family key.


## Must stay out of `workspace_shell_family`
- authority ownership for workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing, authority-facing minimal truth: workspace session ownership only, snapshot classes: workspace snapshots and session-scoped view snapshots, index classes: workspace/session lookup indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `workspace_shell_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
