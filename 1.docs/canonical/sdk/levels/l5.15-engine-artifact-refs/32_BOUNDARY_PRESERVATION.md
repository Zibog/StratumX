# Boundary Preservation

## This level must never own
- must not own mutable state snapshots
- must not mix artifact and state ref semantics
- must not become a hidden artifact store by itself

## Preservation rule
`engine_artifact_refs` exists to keep one semantic class narrow. When a new requirement appears, it must either fit these boundaries or be placed in another declared L5 class.

## Operational note
This file remains active and package-specific for `l5.15-engine-artifact-refs` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
