# Family Local Boundary Preservation

This local family contract belongs specifically to the l7.f3 simulation meta family and may not be reused by a different family key.


## Must stay out of `simulation_meta_family`
- authority ownership for simulation campaign composition and simulation workflow meta, authority-facing minimal truth: simulation campaign refs only, snapshot classes: simulation-campaign snapshots, index classes: simulation-campaign indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `simulation_meta_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
