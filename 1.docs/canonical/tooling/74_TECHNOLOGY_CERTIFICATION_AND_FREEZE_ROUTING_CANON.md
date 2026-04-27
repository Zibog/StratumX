# Technology Certification And Freeze Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for technology certification and freeze.

## Exact intent schema families
- `intent.cert.select_pack`
- `intent.cert.compare_to_baseline`
- `intent.cert.freeze_ready_review`

## Transaction state machine
- selected -> baselined -> executed -> compared -> freeze_reviewed -> frozen / rolled_back

## Cache and artifact ownership
- freeze review cache and certification ledger are authoritative.
- failed runs never become baseline by accident.

## Invalidation triggers
- freeze blocker family change
- baseline pointer change
- artifact loss

## Rollback and retry law
- rollback must target a declared rollback anchor or baseline pointer only;
- retry must retain the failed-run artifact bundle;
- no route may silently normalize away the first failure code.

## Route-level failure families
- `route.cert.freeze_blocker_active`
- `route.cert.baseline_missing`
