# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f7 animation rig family and may not be reused by a different family key.


## Must stay out of `animation_rig_family`
- authority ownership for rig authoring, rig refs, rig validation, and rig manifests, authority-facing minimal truth: rig edit intents, snapshot classes: rig snapshots, index classes: rig lookup indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `animation_rig_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
