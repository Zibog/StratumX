# Frame Viewset Presentation Capture And Result Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the public packet families for frame scope, viewset identity, presentation posture, capture readiness, and terminal frame result.

## Packet families
### `packet.frame.scope.v1`
Required fields:
- `frame_scope_id`
- `world_ref`
- `viewset_ref`
- `camera_ref`
- `frame_intent_class`
- `workspace_stage_id`
- `capture_eligibility_code`

### `packet.frame.presentation_posture.v1`
Required fields:
- `backend_class`
- `shader_target_set`
- `present_path_id`
- `present_surface_id`
- `degrade_rung_code`
- `secondary_view_topology`
- `timing_bucket`

### `packet.frame.capture_result.v1`
Required fields:
- `artifact_ref`
- `baseline_ref`
- `compare_mode_id`
- `result_code`
- `first_blocker_code`
- `reason_trace_ref`

## Compatibility law
- `frame_scope_id`, `viewset_ref`, and `camera_ref` are stable ids and may not be inferred from labels.
- `backend_class` and `present_path_id` are required even when capture fails.
- `secondary_view_topology` must publish `single`, `split_vertical`, `split_horizontal`, or `detached_secondary`.

## Result law
A terminal frame result packet must answer:
- what the operator was looking at;
- which backend/present path was active;
- whether capture was legal;
- which blocker or fallback shaped the result.
