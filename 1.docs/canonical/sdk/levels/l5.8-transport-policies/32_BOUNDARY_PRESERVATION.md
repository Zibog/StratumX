# Boundary Preservation

## This level must never own
- must not own legality reasons or verdicts
- must not own session-local packet queues
- must not encode editor/package concerns

## Preservation rule
`transport_policies` exists to keep one semantic class narrow. When a new requirement appears, it must either fit these boundaries or be placed in another declared L5 class.

## Operational note
This file remains active and package-specific for `l5.8-transport-policies` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
