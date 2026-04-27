# Boundary Preservation

This contract belongs specifically to the optimization reasoner level and may not be reused verbatim by another tooling level.


## Forbidden ownership
- hot mutation ownership
- editor widget state
- undeclared lower-layer truth

## Preservation rule
`optimization_reasoner` stays valid only while it keeps those classes out.

## Operational note
This file remains active and package-specific for `l7a.4-optimization-reasoner` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
