# Hydrology Container Leak And Evaporation Authoring Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose legal authoring, binding, simulation, inspection, compare, capture, recovery, and certification for container continuity, leak topology, hole-level outflow, rainfall fill, and evaporation tiers.
It is a production authoring surface, not a review-only room.

## Command inventory
- `btn.hyd.author_container_profile`
- `btn.hyd.bind_material_response`
- `btn.hyd.inspect_mass_ledger`
- `btn.hyd.simulate_rainfall_burst`
- `btn.hyd.compare_triplet`
- `btn.hyd.capture_evidence`
- `btn.hyd.recover_baseline`
- `btn.hyd.certify_pack`

## Overlay families
- `overlay.hyd.mass_ledger`
- `overlay.hyd.leak_topology`
- `overlay.hyd.pool_depth`
- `overlay.hyd.hole_level_lock`
- `overlay.hyd.restore_triplet`
- `overlay.hyd.failure_board`

## Inspector fields
- `container_profile_id`
- `material_response_id`
- `mass_ledger_state`
- `leak_aperture_state`
- `hole_level_lock_state`
- `evaporation_tier_state`
- `restore_anchor_ref`
- `artifact_ref`

## Disabled reasons
- `disable.hyd.author_blocked`
- `disable.hyd.bind_blocked`
- `disable.hyd.inspect_blocked`
- `disable.hyd.capture_blocked`

## Exact brutal proof slice
`rainfall fill -> shot leak -> outflow until hole level -> leak stop -> evaporation -> restore/replay compare`

## Compare and capture law
This lab may use only `compare.hyd.restore_triplet` and `capture.hyd.ledger_bundle` for `pack.hydrology_persistence`.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open`
