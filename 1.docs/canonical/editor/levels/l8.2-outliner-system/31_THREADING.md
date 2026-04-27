# Threading

This contract belongs specifically to the outliner system editor level and is expected to become direct implementation work.


## Threading posture for `outliner_system`
- interactive ownership of `outliner_view_id`, `hierarchy_projection_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to row selection publications; hierarchy refreshes publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l8.2-outliner-system` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
