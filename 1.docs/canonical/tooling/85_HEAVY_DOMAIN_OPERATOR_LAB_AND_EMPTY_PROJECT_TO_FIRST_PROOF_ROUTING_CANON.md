# Heavy Domain Operator Lab And Empty Project To First Proof Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact routing spine from empty project to first retained heavy-domain proof through editor labs.

## Route stages
| Stage | Primary handoff | Required artifact | Failure family |
|---|---|---|---|
| project bootstrap | editor shell and project state | project identity | `PFR_PROJECT_*` |
| base world authoring | terrain/material/sky baseline | world baseline snapshot | `PFR_WORLD_*`, `PFR_TERRAIN_*`, `PFR_CLIMATE_*` |
| domain-lab mutation | heavy-domain transaction request | tx/result ledger | `TRN_*`, `DST_*`, `HYD_*`, `LIV_*`, `RND_*` |
| play/sim proof | runtime trace and event window | capture bundle | `PFR_SLICE_*` |
| compare/certify | baseline vs run verdict | compare/cert bundle | `PFR_COMPARE_*`, `PFR_CERT_*` |

## Invalidations
A route must mark stale captures, stale compares, stale certification verdicts, and stale release-readiness rows when the proof lane mutates.

## Focus return law
A blocker must return to the narrowest originating lab, not a generic dashboard.
