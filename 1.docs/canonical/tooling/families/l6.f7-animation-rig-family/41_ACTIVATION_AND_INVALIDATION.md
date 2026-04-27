# Activation And Invalidation

This contract belongs specifically to the l6.f7 animation rig family and describes family-only coordination.


## Activation posture for `animation_rig_family`
- the family may warm members together when rig authoring, rig refs, rig validation, and rig manifests and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in rig authoring, rig refs, rig validation, and rig manifests, authority-facing minimal truth: rig edit intents, snapshot classes: rig snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f7-animation-rig-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
