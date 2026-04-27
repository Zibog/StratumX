# Link Ingress Controls Local Threading

## Threading law
- submission is ordered by source session and control domain; acceptance may validate concurrently but publication order is serialized per session
- no publication reordering across the owning cursor or session
- no mutation of already-published `link_ingress_controls` records

## Operational note
This file remains active and package-specific for `link_ingress_controls` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
