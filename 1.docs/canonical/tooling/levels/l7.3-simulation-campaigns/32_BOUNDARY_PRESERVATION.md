# Boundary Preservation

This contract belongs specifically to the simulation campaigns level and may not be reused verbatim by another tooling level.


## Forbidden ownership
- hot mutation ownership
- editor widget state
- undeclared lower-layer truth

## Preservation rule
`simulation_campaigns` stays valid only while it keeps those classes out.

## Operational note
This file remains active and package-specific for `l7.3-simulation-campaigns` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
