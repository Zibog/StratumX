# Threading

This contract belongs specifically to the package market and dependency service editor level and is expected to become direct implementation work.


## Threading posture for `package_market_and_dependency_service`
- interactive ownership of `package_service_session_id`, `package_listing_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to package search and action requests; dependency graph refreshes publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l10.7-package-market-and-dependency-service` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
