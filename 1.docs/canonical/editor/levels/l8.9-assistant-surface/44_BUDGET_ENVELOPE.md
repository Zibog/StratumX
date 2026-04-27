# Budget Envelope

This contract belongs specifically to the assistant surface editor level and is expected to become direct implementation work.


## Budget rule for `assistant_surface`
- correctness of `assistant_surface_id`, `conversation_session_id` and declared request legality is non-degradable
- previews or speculative work such as assistant prompt submits; proposal review updates degrade first
- deferred work for `{key}` must remain visible, queued, or cancellable rather than hidden

## Operational note
This file remains active and package-specific for `l8.9-assistant-surface` / `44_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 44 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
