# Boundary Preservation

## This level must never own
- must not carry capability facts inside version rows
- must not own transport policy or verdict truth
- must not depend on editor/tooling state

## Preservation rule
`compat_versions` exists to keep one semantic class narrow. When a new requirement appears, it must either fit these boundaries or be placed in another declared L5 class.

## Operational note
This file remains active and package-specific for `l5.4-compat-versions` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
