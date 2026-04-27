# Budget Envelope

This contract belongs specifically to the scene entity authoring suite editor level and is expected to become direct implementation work.


## Budget rule for `scene_entity_authoring_suite`
- correctness of `scene_suite_session_id`, `entity_selection_ref_set` and declared request legality is non-degradable
- previews or speculative work such as entity mutation requests; Prefab Apply/Revert / diff / unpack requests degrade first
- deferred work for `{key}` must remain visible, queued, or cancellable rather than hidden

## Operational note
This file remains active and package-specific for `l9.1-scene-entity-authoring-suite` / `44_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 44 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
