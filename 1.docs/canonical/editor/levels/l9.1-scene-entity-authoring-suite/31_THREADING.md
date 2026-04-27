# Threading

This contract belongs specifically to the scene entity authoring suite editor level and is expected to become direct implementation work.


## Threading posture for `scene_entity_authoring_suite`
- interactive ownership of `scene_suite_session_id`, `entity_selection_ref_set` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to entity mutation requests; Prefab Apply/Revert / diff / unpack requests publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l9.1-scene-entity-authoring-suite` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
