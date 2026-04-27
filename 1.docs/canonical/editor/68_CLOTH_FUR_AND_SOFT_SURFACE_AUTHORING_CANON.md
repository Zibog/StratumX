# Cloth Fur And Soft Surface Authoring Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose legal authoring and preview for cloth, fur, wind pressure, wetness response, contact solve, cheap coverage richness, promoted-local strand-like mode, and recovery under solver pressure.
It is a production authoring surface, not a review-only room.

## Command inventory
- `btn.soft.author_cloth_profile`
- `btn.soft.bind_surface_group`
- `btn.soft.inspect_contact_state`
- `btn.soft.preview_contact`
- `btn.soft.compare_triplet`
- `btn.soft.capture_evidence`
- `btn.soft.recover_baseline`
- `btn.soft.certify_pack`

## Overlay families
- `overlay.soft.profile`
- `overlay.soft.contact_state`
- `overlay.soft.wind_pressure`
- `overlay.soft.coverage_richness`
- `overlay.soft.recovery_triplet`
- `overlay.soft.failure_board`

## Inspector fields
- `cloth_profile_id`
- `surface_group_id`
- `contact_state`
- `solver_rung`
- `coverage_profile_id`
- `coverage_repr_id`
- `promoted_local_mode`
- `baseline_ref`
- `artifact_ref`

## Disabled reasons
- `disable.soft.author_blocked`
- `disable.soft.bind_blocked`
- `disable.soft.inspect_blocked`
- `disable.soft.capture_blocked`

## Compare and capture law
This lab may use only `compare.soft.contact_triplet` and `capture.soft.contact_bundle` for `pack.fur_cloth_weather`.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Coverage law
This lab may author fur/hair-like coverage posture only as a declared coverage profile under root `96` and engine `106`.
It may preview promoted-local richness.
It may not require promoted-local richness as the only legal result.

## Post-action focus rules
- success -> editor/96 or editor/105
- retryable failure -> editor/100
- terminal failure -> editor/100 then editor/109
- recovery -> owner lab then compare
