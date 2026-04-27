# Budget Profile Fallback And Degrade Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the public bridge for budget posture, degrade ladders, fallback publication, and old-floor visibility.

## Packet families
### `packet.budget.active_profile.v1`
Required fields:
- `budget_profile_id`
- `hardware_floor_id`
- `frame_budget_ms`
- `memory_budget_mb`
- `secondary_view_budget_state`

### `packet.budget.degrade_step.v1`
Required fields:
- `degrade_rung_code`
- `trigger_code`
- `affected_subsystems`
- `visible_badge_set`
- `recovery_trigger_id`

### `packet.budget.fallback_result.v1`
Required fields:
- `backend_fallback_code`
- `capture_fallback_code`
- `viewport_throttle_code`
- `operator_legality_code`

## Law
Fallback publication must remain explicit even when the result is still “good enough” for authoring.
