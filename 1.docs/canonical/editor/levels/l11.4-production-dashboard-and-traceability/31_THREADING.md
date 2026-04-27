# Threading

This contract belongs specifically to the production dashboard and traceability editor level and is expected to become direct implementation work.


## Threading posture for `production_dashboard_and_traceability`
- interactive ownership of `dashboard_surface_id`, `traceability_view_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to dashboard refreshes; drill-down selections publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l11.4-production-dashboard-and-traceability` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
