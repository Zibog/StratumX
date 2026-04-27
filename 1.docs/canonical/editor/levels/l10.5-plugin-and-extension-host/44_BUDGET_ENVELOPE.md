# Budget Envelope

This contract belongs specifically to the plugin and extension host editor level and is expected to become direct implementation work.


## Budget rule for `plugin_and_extension_host`
- correctness of `plugin_host_session_id`, `registered_dock_set` and declared request legality is non-degradable
- previews or speculative work such as plugin registration requests; extension lifecycle publications degrade first
- deferred work for `{key}` must remain visible, queued, or cancellable rather than hidden

## Operational note
This file remains active and package-specific for `l10.5-plugin-and-extension-host` / `44_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 44 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
