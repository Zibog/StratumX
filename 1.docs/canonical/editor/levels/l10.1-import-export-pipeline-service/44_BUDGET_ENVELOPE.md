# Budget Envelope

This contract belongs specifically to the import export pipeline service editor level and is expected to become direct implementation work.


## Budget rule for `import_export_pipeline_service`
- correctness of `pipeline_request_id`, `source_path_ref` and declared request legality is non-degradable
- previews or speculative work such as import/reimport/export requests; generated file reveal publications degrade first
- deferred work for `{key}` must remain visible, queued, or cancellable rather than hidden

## Operational note
This file remains active and package-specific for `l10.1-import-export-pipeline-service` / `44_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 44 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
