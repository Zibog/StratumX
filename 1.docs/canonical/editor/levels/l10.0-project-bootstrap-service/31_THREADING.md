# Threading

This contract belongs specifically to the project bootstrap service editor level and is expected to become direct implementation work.


## Threading posture for `project_bootstrap_service`
- interactive ownership of `bootstrap_request_id`, `project_manifest_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to new/open project requests; mount and seed plan publications publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l10.0-project-bootstrap-service` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
