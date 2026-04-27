# Threading

This contract belongs specifically to the content browser system editor level and is expected to become direct implementation work.


## Threading posture for `content_browser_system`
- interactive ownership of `content_browser_view_id`, `asset_listing_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to asset listing refreshes; dependency/reference queries publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l8.3-content-browser-system` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
