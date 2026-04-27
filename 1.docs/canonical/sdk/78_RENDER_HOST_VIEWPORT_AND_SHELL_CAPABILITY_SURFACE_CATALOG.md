# Render Host Viewport And Shell Capability Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the public bridge for viewport-host, shell, and present-capability publication used by tooling and editor shell layers.

## Packet families
### `packet.shell.viewport_capability.v1`
Required fields:
- `primary_viewport_id`
- `split_view_supported`
- `detached_view_supported`
- `capture_supported`
- `max_concurrent_view_count`

### `packet.shell.layout_state.v1`
Required fields:
- `layout_id`
- `workspace_stage_id`
- `focused_surface_id`
- `detached_window_count`
- `layout_dirty_flag`

### `packet.shell.present_capability.v1`
Required fields:
- `present_surface_id`
- `backend_class`
- `secondary_view_budget_state`
- `honest_viewport_code`
- `present_failure_code`

## Law
These packets expose shell capability only.
They do not make the shell an owner of render truth.
