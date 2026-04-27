# Activation And Invalidation

This contract belongs specifically to the l6.f8 animation motion authoring family and describes family-only coordination.


## Activation posture for `animation_motion_authoring_family`
- the family may warm members together when motion authoring, clip graphs, motion rules, and motion previews and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in motion authoring, clip graphs, motion rules, and motion previews, authority-facing minimal truth: motion edit intents, snapshot classes: motion snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f23-animation-motion-authoring-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
