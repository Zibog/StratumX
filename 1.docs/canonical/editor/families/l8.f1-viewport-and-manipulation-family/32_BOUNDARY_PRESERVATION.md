# Boundary Preservation

This contract belongs specifically to the l8.f1 viewport and manipulation family and is not interchangeable with another editor family.


## Must stay out of `viewport_and_manipulation_family`
- member-level truth ownership for viewport hosts, navigation, overlays, gizmos, snapping, and focused manipulation context, viewport projections, tool contexts, diagnostics overlays, manipulation and viewport requests
- lower-package mutable authority
- unrelated editor domains

## Operational note
This file remains active and package-specific for `l8.f1-viewport-and-manipulation-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
