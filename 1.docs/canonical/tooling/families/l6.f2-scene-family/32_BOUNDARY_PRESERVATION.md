# Boundary Preservation

This contract belongs specifically to the l6.f2 scene family and describes family-only coordination.


## Must remain outside `scene_family` ownership
- member-internal mutable authority rows for scene graph authoring, entity grouping, scene views, and scene manifests, authority-facing minimal truth: scene edit intents and scene root refs, snapshot classes: scene snapshots, index classes: scene/spatial lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f2-scene-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
