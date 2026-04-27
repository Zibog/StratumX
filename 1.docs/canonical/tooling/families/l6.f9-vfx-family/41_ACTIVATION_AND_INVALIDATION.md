# Activation And Invalidation

This contract belongs specifically to the l6.f9 vfx family and describes family-only coordination.


## Activation posture for `vfx_family`
- the family may warm members together when VFX authoring, effect graphs, effect views, and VFX manifests and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in VFX authoring, effect graphs, effect views, and VFX manifests, authority-facing minimal truth: VFX edit intents, snapshot classes: VFX snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f9-vfx-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
