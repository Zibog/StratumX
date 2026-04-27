# Budget Envelope

This contract belongs specifically to the overlay and gizmo system editor level and is expected to become direct implementation work.


## Budget rule for `overlay_and_gizmo_system`
- correctness of `overlay_set_id`, `gizmo_state_id` and declared request legality is non-degradable
- previews or speculative work such as overlay redraw requests; gizmo interaction results degrade first
- deferred work for `{key}` must remain visible, queued, or cancellable rather than hidden

## Operational note
This file remains active and package-specific for `l8.6-overlay-and-gizmo-system` / `44_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 44 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
