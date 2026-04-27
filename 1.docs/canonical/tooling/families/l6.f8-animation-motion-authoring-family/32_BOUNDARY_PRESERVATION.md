# Boundary Preservation

This contract belongs specifically to the l6.f8 animation motion authoring family family and describes family-only coordination.


## Must remain outside `animation_motion_authoring_family` ownership
- member-internal mutable authority rows for motion authoring, clip graphs, motion rules, and motion previews, authority-facing minimal truth: motion edit intents, snapshot classes: motion snapshots, index classes: motion lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f8-animation-motion-authoring-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
