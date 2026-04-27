# Boundary Preservation

## This level must never own
- must not emit verdicts directly
- must not own transport retry or sampling policy
- must not contain session-local mutable fields

## Preservation rule
`compat_profiles` exists to keep one semantic class narrow. When a new requirement appears, it must either fit these boundaries or be placed in another declared L5 class.

## Operational note
This file remains active and package-specific for `l5.6-compat-profiles` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
