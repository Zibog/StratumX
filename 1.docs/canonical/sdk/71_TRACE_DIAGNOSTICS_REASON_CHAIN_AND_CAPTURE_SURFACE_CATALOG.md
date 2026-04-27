# Trace Diagnostics Reason Chain And Capture Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the public bridge for reason traces, diagnostics drilldown, and retained capture lineage.

## Packet families
### `packet.trace.reason_summary.v1`
Required fields:
- `trace_id`
- `owner_package_id`
- `primary_reason_code`
- `next_action_id`
- `focus_target_id`

### `packet.trace.blocker_triplet.v1`
Required fields:
- `failed_run_artifact_ref`
- `baseline_artifact_ref`
- `recovered_artifact_ref`
- `first_blocker_code`
- `recovery_result_code`

### `packet.trace.apply_revert_lineage.v1`
Required fields:
- `proposal_id`
- `applied_route_set`
- `rollback_anchor_id`
- `artifact_ref`
- `revert_eligibility_code`

## Law
Reason-chain packets must support ordinary render failures, assistant applies/reverts, and extension-assisted actions through one vocabulary.
