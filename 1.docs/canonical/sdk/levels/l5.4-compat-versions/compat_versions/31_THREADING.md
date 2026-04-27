# Compatibility Versions Local Threading

## Threading law
- single-writer registry with immutable published rows; readers may cache lookups only outside L5
- no publication reordering across the owning cursor or session
- no mutation of already-published `compat_versions` records

## Operational note
This file remains active and package-specific for `compat_versions` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
