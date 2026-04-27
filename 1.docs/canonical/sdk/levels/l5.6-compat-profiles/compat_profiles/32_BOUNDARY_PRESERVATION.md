# Compatibility Profiles Local Boundary Preservation

## Must stay out of this level
- must not emit verdicts directly
- must not own transport retry or sampling policy
- must not contain session-local mutable fields

## Local preservation rule
`compat_profiles` is valid only while it stays narrower than adjacent bridge classes.

## Operational note
This file remains active and package-specific for `compat_profiles` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
