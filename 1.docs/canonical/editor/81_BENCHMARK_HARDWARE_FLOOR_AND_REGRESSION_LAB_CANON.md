# Benchmark Hardware Floor And Regression Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose legal authoring for threshold profiles, threshold-table binding, axis-red inspection, floor-pack simulation, compare, capture, recovery, certification, and regression review.
It is a production authoring surface, not a review-only room.

## Command inventory
- `btn.hardware.author_profile`
- `btn.hardware.bind_threshold_table`
- `btn.hardware.inspect_axis_red`
- `btn.hardware.simulate_floor_pack`
- `btn.hardware.compare_floor_pack`
- `btn.hardware.capture_floor_evidence`
- `btn.hardware.recover_last_good`
- `btn.hardware.certify_floor_pack`

## Surface obligations
This lab must also expose:
- mixed-pack regression history for the same retained bundle chain;
- the actual rung taken per domain;
- one explicit pointer to the last-good baseline;
- one explicit pointer to the first blocking failure for the current run.

## Overlay families
- `overlay.hardware.profile`
- `overlay.hardware.threshold_table`
- `overlay.hardware.axis_pressure`
- `overlay.hardware.floor_triplet`
- `overlay.hardware.failure_board`
- `overlay.hardware.regression_matrix`

## Inspector fields
- `hardware_profile_id`
- `threshold_table_id`
- `axis_pressure_state`
- `floor_triplet_ref`
- `baseline_ref`
- `artifact_ref`
- `regression_run_ref`
- `actual_rung_vector`

## Disabled reasons
- `disable.hardware.author_blocked`
- `disable.hardware.bind_blocked`
- `disable.hardware.inspect_blocked`
- `disable.hardware.capture_blocked`

## Compare and capture law
This lab may use only `compare.hardware.floor_triplet` and `capture.hardware.floor_bundle` for `pack.combined_old_hardware_floor`.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Post-action focus rules
- success -> editor `74` or editor `105`;
- retryable failure -> editor `100`;
- terminal failure -> editor `100` then editor `109`;
- recovery -> owner lab then compare;
- regression blocker -> editor `87` or editor `89` depending on whether the blocker is freeze-local or scale-local.
