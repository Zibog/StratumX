# Compatibility Profiles Local Threading

## Threading law
- single-writer immutable registry
- no publication reordering across the owning cursor or session
- no mutation of already-published `compat_profiles` records

## Operational note
This file remains active and package-specific for `compat_profiles` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
