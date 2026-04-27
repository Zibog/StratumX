# Play Simulate Fix And Retest Operator Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose the production loop for play slices, reason inspection, triplet compare, evidence capture, and certification handoff.
It is a production authoring surface, not a review-only room.
In phase 5 this lab is the hinge between a local scenario run and the retained brutal proof-region relay bundle.

## Command inventory
- `btn.sim.play_slice`
- `btn.inspect.reason_chain`
- `btn.compare.triplet`
- `btn.capture.evidence`
- `btn.certify.pack`

## Overlay families
- `overlay.play.slice`
- `overlay.reason.chain`
- `overlay.compare.triplet`
- `overlay.capture.bundle`
- `overlay.cert.handoff`
- `overlay.proof_region.relay`

## Inspector fields
- `play_slice_id`
- `proof_region_recipe_ref`
- `baseline_run_ref`
- `failed_run_ref`
- `recovery_run_ref`
- `compare_triplet_ref`
- `capture_bundle_ref`
- `cert_handoff_ref`

## Disabled reasons
- `disable.sim.play_blocked`
- `disable.reason.inspect_blocked`
- `disable.compare.triplet_blocked`
- `disable.capture.evidence_blocked`
- `disable.relay.bundle_incomplete`

## Compare and capture law
This lab may use only the scenario-selected compare and capture modes routed by the owning pack row in `editor/110`.
When the active pack is `pack.brutal_proof_region_relay`, the lab must retain baseline, failed, and recovery runs as separate bundle members.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Post-action focus rules
- success -> owner lab, `editor/103`, or `editor/109` according to the active button row in `editor/110`;
- retryable failure -> `editor/100` with the exact next legal recovery action pinned;
- terminal failure -> `editor/100` then `editor/109` with blocker board context;
- recovery -> rerun compare or hand control back to the owning lab recover row.
