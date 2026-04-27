# Budget Envelope

This contract belongs specifically to the snapshot plane level and may not be reused verbatim by another tooling level.


## Budget posture for `snapshot_plane`
- correctness of `snapshot_id`, `source_epoch` is non-degradable
- lower-priority outputs such as immutable snapshots for index/derived/preview/runtime consumers degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.3-snapshot-plane` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
