# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f11 ui family and may not be reused by a different family key.


## Must stay out of `ui_family`
- authority ownership for UI authoring, widget surfaces, UI schemas, and UI manifests, authority-facing minimal truth: UI edit intents, snapshot classes: UI snapshots, index classes: UI lookup indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `ui_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
