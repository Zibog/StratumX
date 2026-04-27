# Test Surfaces

This contract belongs specifically to the l6.f1 content family and describes family-only coordination.


## Required checks for `content_family`
- composition-only legality across content browser, asset identity, import/reimport flows, dependency and reverse-reference views, content manifests, authority-facing minimal truth: minimal asset refs and edit intents, snapshot classes: content snapshots
- dependency legality against member contracts
- activation and invalidation propagation for the declared family members

## Operational note
This file remains active and package-specific for `l6.f1-content-family` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
