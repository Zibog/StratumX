# Test Surfaces

This contract belongs specifically to the l6a.f1 assistant safety family and describes family-only coordination.


## Required checks for `assistant_safety_family`
- composition-only legality across assistant safety, approval, legality, and apply/revert policy composition, authority-facing minimal truth: safety/approval refs only, snapshot classes: safety snapshots
- dependency legality against member contracts
- activation and invalidation propagation for the declared family members

## Operational note
This file remains active and package-specific for `l6a.f1-assistant-safety-family` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
