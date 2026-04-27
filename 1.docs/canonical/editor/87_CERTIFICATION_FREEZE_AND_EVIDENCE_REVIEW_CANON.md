# Certification Freeze And Evidence Review Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose freeze review, signoff, waiver visibility, and one-rung recovery as legal production actions tied to retained certification artifacts.
It is a production authoring surface, not a review-only room.

## Command inventory
- `btn.freeze.review_bundle`
- `btn.freeze.signoff`
- `btn.recover.drop_one_rung`
- `btn.trace.reason.inspect`

## Overlay families
- `overlay.freeze.review`
- `overlay.freeze.signoff`
- `overlay.floor.recover_rung`
- `overlay.cert.bundle`
- `overlay.freeze.failure_board`
- `overlay.proof_region.release_chain`
- `overlay.freeze.blocker_matrix`

## Inspector fields
- `review_bundle_ref`
- `signoff_ref`
- `recovery_rung_ref`
- `cert_bundle_ref`
- `blocker_trace_ref`
- `proof_region_recipe_ref`
- `first_result_verification_ref`
- `waiver_state`
- `old_floor_result_ref`

## Disabled reasons
- `disable.freeze.review_blocked`
- `disable.freeze.signoff_blocked`
- `disable.recover.rung_blocked`
- `disable.freeze.launch_proof_missing`

## Freeze law
Freeze is blocked when any of the following is missing:
- retained baseline;
- compare digest;
- blocker trace;
- required failed-run and recovery-run bundles;
- required old-floor result or typed exemption;
- first-result verification when a product relay is involved.

## Compare and capture law
This lab may use only `compare.cert.pack` and `capture.cert.bundle` for the active certification or release pack under review.
For release closure the canonical packs are `pack.release_freeze_signoff` and `pack.brutal_proof_region_relay`; for old-floor fallback it may also review `pack.combined_old_hardware_floor`.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Post-action focus rules
- success -> editor `109`;
- retryable failure -> editor `81` or editor `100`;
- terminal failure -> editor `109` with blocker trace and retained baseline lineage;
- recovery -> rerun compare before signoff.
