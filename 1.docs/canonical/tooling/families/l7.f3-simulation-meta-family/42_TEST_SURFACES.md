# Test Surfaces

This contract belongs specifically to the l7.f3 simulation meta family and describes family-only coordination.


## Required checks for `simulation_meta_family`
- composition-only legality across simulation campaign composition and simulation workflow meta, authority-facing minimal truth: simulation campaign refs only, snapshot classes: simulation-campaign snapshots
- dependency legality against member contracts
- activation and invalidation propagation for the declared family members

## Operational note
This file remains active and package-specific for `l7.f3-simulation-meta-family` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
