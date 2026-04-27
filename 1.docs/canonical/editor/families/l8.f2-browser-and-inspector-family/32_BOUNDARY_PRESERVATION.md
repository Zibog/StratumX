# Boundary Preservation

This contract belongs specifically to the l8.f2 browser and inspector family and is not interchangeable with another editor family.


## Must stay out of `browser_and_inspector_family`
- member-level truth ownership for outliner, content browser, details/inspector, and metadata-heavy product surfaces, snapshot and index projections, diagnostics hints, focus, reveal, staged edit, and browser requests
- lower-package mutable authority
- unrelated editor domains

## Operational note
This file remains active and package-specific for `l8.f2-browser-and-inspector-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
