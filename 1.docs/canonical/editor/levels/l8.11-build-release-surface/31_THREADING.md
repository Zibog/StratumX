# Threading

This contract belongs specifically to the build release surface editor level and is expected to become direct implementation work.


## Threading posture for `build_release_surface`
- interactive ownership of `build_release_surface_id`, `build_status_view_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to build/release action requests; status refreshes publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l8.11-build-release-surface` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
