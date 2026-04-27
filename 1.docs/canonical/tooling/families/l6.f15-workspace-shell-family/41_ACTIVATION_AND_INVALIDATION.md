# Activation And Invalidation

This contract belongs specifically to the l6.f0 workspace shell family and describes family-only coordination.


## Activation posture for `workspace_shell_family`
- the family may warm members together when workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing, authority-facing minimal truth: workspace session ownership only, snapshot classes: workspace snapshots and session-scoped view snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f15-workspace-shell-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
