# Data Responsibility

This contract belongs specifically to the l8.f1 viewport and manipulation family and is not interchangeable with another editor family.


## Family-scoped data for `viewport_and_manipulation_family`
- coordination data for viewport hosts, navigation, overlays, gizmos, snapping, and focused manipulation context
- coordination data for viewport projections, tool contexts, diagnostics overlays
- coordination data for manipulation and viewport requests

## Note
Family-scoped data stays descriptive and coordinating; member levels keep the operational truth.

## Operational note
This file remains active and package-specific for `l8.f1-viewport-and-manipulation-family` / `40_DATA_RESPONSIBILITY.md`.

## Scope note
The authority, dependency, and audit meaning of 40 DATA RESPONSIBILITY is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
