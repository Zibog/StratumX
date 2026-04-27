# Engine Session Handles Local Threading

## Threading law
- single writer for status transitions per handle; many readers may resolve immutable metadata
- no publication reordering across the owning cursor or session
- no mutation of already-published `engine_session_handles` records

## Operational note
This file remains active and package-specific for `engine_session_handles` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
