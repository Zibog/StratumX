# Boundary Preservation

## This level must never own
- must not mutate source profiles or capabilities
- must not own long-lived policy registries
- must not emit editor-facing prose as the verdict body

## Preservation rule
`compat_verdicts` exists to keep one semantic class narrow. When a new requirement appears, it must either fit these boundaries or be placed in another declared L5 class.

## Operational note
This file remains active and package-specific for `l5.7-compat-verdicts` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
