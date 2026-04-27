# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f6 simulation debug family and may not be reused by a different family key.


## Must stay out of `simulation_debug_family`
- authority ownership for simulation debug views, probes, timeline views, and diagnostic overlays, authority-facing minimal truth: no domain authority beyond debug-session refs, snapshot classes: simulation debug snapshots, index classes: simulation debug indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `simulation_debug_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
