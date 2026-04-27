# Boundary Preservation

## Must stay out of this level
- raw filesystem watcher ownership
- runtime asset truth
- hidden import queues

## Preservation rule
`import_export_pipeline_service` exists to keep this editor concern local instead of leaking it into sdk, tooling, or another editor level.

## Operational note
This file remains active and package-specific for `l10.1-import-export-pipeline-service` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
