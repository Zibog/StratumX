# Boundary Preservation

This contract belongs specifically to the l6.f1 content family and describes family-only coordination.


## Must remain outside `content_family` ownership
- member-internal mutable authority rows for content browser, asset identity, import/reimport flows, dependency and reverse-reference views, content manifests, authority-facing minimal truth: minimal asset refs and edit intents, snapshot classes: content snapshots, index classes: content lookup, dependency, reverse-dependency, and search indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f1-content-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
