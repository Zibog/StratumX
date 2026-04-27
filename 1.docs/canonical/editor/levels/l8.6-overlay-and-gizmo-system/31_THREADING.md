# Threading

This contract belongs specifically to the overlay and gizmo system editor level and is expected to become direct implementation work.


## Threading posture for `overlay_and_gizmo_system`
- interactive ownership of `overlay_set_id`, `gizmo_state_id` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to overlay redraw requests; gizmo interaction results publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l8.6-overlay-and-gizmo-system` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
