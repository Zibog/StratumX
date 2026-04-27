# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f1 content family and may not be reused by a different family key.


## Must stay out of `content_family`
- authority ownership for content browser, asset identity, import/reimport flows, dependency and reverse-reference views, content manifests, authority-facing minimal truth: minimal asset refs and edit intents, snapshot classes: content snapshots, index classes: content lookup, dependency, reverse-dependency, and search indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `content_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
