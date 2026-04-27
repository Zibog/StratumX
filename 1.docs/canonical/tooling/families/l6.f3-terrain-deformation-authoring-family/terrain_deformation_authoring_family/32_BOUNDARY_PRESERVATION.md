# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f3 terrain deformation authoring family family and may not be reused by a different family key.


## Must stay out of `terrain_deformation_authoring_family`
- authority ownership for terrain layers, deformation intents, brush results, and terrain manifests, authority-facing minimal truth: terrain edit intents only, snapshot classes: terrain snapshots, index classes: terrain indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `terrain_deformation_authoring_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
