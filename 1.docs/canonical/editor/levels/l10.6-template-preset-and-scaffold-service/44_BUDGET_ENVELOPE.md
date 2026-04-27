# Budget Envelope

This contract belongs specifically to the template preset and scaffold service editor level and is expected to become direct implementation work.


## Budget rule for `template_preset_and_scaffold_service`
- correctness of `scaffold_request_id`, `template_ref` and declared request legality is non-degradable
- previews or speculative work such as template apply requests; generated item publications degrade first
- deferred work for `{key}` must remain visible, queued, or cancellable rather than hidden

## Operational note
This file remains active and package-specific for `l10.6-template-preset-and-scaffold-service` / `44_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 44 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
