# Engine Artifact Refs Local Boundary Preservation

## Must stay out of this level
- must not own mutable state snapshots
- must not mix artifact and state ref semantics
- must not become a hidden artifact store by itself

## Local preservation rule
`engine_artifact_refs` is valid only while it stays narrower than adjacent bridge classes.

## Operational note
This file remains active and package-specific for `engine_artifact_refs` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
