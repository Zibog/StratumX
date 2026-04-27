# Dependencies

This contract belongs specifically to the l8.f1 viewport and manipulation family and is not interchangeable with another editor family.


## Family dependency posture for `viewport_and_manipulation_family`
- member-local coordination for viewport hosts, navigation, overlays, gizmos, snapping, and focused manipulation context
- member-local coordination for viewport projections, tool contexts, diagnostics overlays
- member-local coordination for manipulation and viewport requests
- lower packages only through member-legal public surfaces
- no family-local authority shortcut

## Operational note
This file remains active and package-specific for `l8.f1-viewport-and-manipulation-family` / `20_DEPENDENCIES.md`.

## Scope note
The authority, dependency, and audit meaning of 20 DEPENDENCIES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
