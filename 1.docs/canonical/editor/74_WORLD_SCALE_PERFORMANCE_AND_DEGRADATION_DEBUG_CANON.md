# World Scale Performance And Degradation Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose legal authoring for budget profiles, domain mixes, pressure-ladder inspection, mixed-pack simulation, compare, capture, recovery, and certification on the old-hardware floor.
It is a production authoring surface, not a review-only room.

## Command inventory
- `btn.worldfloor.author_budget_profile`
- `btn.worldfloor.bind_domain_mix`
- `btn.worldfloor.inspect_pressure_ladder`
- `btn.worldfloor.simulate_mixed_pack`
- `btn.worldfloor.compare_pack`
- `btn.worldfloor.capture_evidence`
- `btn.worldfloor.recover_last_good`
- `btn.worldfloor.certify_pack`

## Overlay families
- `overlay.worldfloor.budget_profile`
- `overlay.worldfloor.pressure_ladder`
- `overlay.worldfloor.degrade_rungs`
- `overlay.worldfloor.pack_triplet`
- `overlay.worldfloor.failure_board`

## Inspector fields
- `budget_profile_id`
- `domain_mix_id`
- `pressure_ladder_state`
- `pack_triplet_ref`
- `baseline_ref`
- `artifact_ref`

## Disabled reasons
- `disable.worldfloor.author_blocked`
- `disable.worldfloor.bind_blocked`
- `disable.worldfloor.inspect_blocked`
- `disable.worldfloor.capture_blocked`

## Compare and capture law
This lab may use only `compare.worldfloor.pack_triplet` and `capture.worldfloor.floor_bundle` for `pack.combined_old_hardware_floor`.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Post-action focus rules
- success -> editor/81 or editor/105
- retryable failure -> editor/100
- terminal failure -> editor/100 then editor/109
- recovery -> owner lab then compare
