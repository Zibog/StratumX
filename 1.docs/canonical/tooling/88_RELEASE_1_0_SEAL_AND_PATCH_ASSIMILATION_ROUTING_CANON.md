# Release 1.0 Seal And Patch Assimilation Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the operational route for turning a patch-wave into an assimilated active contour.

## Required checks
- index sync
- ledger sync
- coverage sync
- readiness sync
- package companion sync
- open-truth review
- evidence registry sync
- technology-description sync

## Route law
A patch-wave is not sealed until the release-seal route emits one `packet.release_seal_review.v1` with no failed sync rows.

## Required outputs
- one seal verdict
- one failing row list if blocked
- one focus-return map to the fixing surfaces
