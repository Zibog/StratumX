# Boundary Preservation

This contract belongs specifically to the derived plane level and may not be reused verbatim by another tooling level.


## Forbidden ownership
- authoritative mutable state
- hidden caches beyond cache_plane
- editor shell ownership

## Preservation rule
`derived_plane` stays valid only while it keeps those classes out.

## Operational note
This file remains active and package-specific for `l6.5-derived-plane` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
