# Dependencies

This contract belongs specifically to the l8.f2 browser and inspector family and is not interchangeable with another editor family.


## Family dependency posture for `browser_and_inspector_family`
- member-local coordination for outliner, content browser, details/inspector, and metadata-heavy product surfaces
- member-local coordination for snapshot and index projections, diagnostics hints
- member-local coordination for focus, reveal, staged edit, and browser requests
- lower packages only through member-legal public surfaces
- no family-local authority shortcut

## Operational note
This file remains active and package-specific for `l8.f2-browser-and-inspector-family` / `20_DEPENDENCIES.md`.

## Scope note
The authority, dependency, and audit meaning of 20 DEPENDENCIES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
