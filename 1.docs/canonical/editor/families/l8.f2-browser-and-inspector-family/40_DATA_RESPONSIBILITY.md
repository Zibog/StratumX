# Data Responsibility

This contract belongs specifically to the l8.f2 browser and inspector family and is not interchangeable with another editor family.


## Family-scoped data for `browser_and_inspector_family`
- coordination data for outliner, content browser, details/inspector, and metadata-heavy product surfaces
- coordination data for snapshot and index projections, diagnostics hints
- coordination data for focus, reveal, staged edit, and browser requests

## Note
Family-scoped data stays descriptive and coordinating; member levels keep the operational truth.

## Operational note
This file remains active and package-specific for `l8.f2-browser-and-inspector-family` / `40_DATA_RESPONSIBILITY.md`.

## Scope note
The authority, dependency, and audit meaning of 40 DATA RESPONSIBILITY is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
