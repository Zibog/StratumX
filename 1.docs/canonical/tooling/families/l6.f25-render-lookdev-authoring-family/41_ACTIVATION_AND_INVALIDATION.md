# Activation And Invalidation

This contract belongs specifically to the l6.f10 render lookdev authoring family and describes family-only coordination.


## Activation posture for `render_lookdev_authoring_family`
- the family may warm members together when render lookdev authoring, lighting/look rules, and render previews and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in render lookdev authoring, lighting/look rules, and render previews, authority-facing minimal truth: lookdev edit intents, snapshot classes: lookdev snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f25-render-lookdev-authoring-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
