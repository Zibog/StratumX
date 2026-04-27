# Budget Envelope

This contract belongs specifically to the index plane level and may not be reused verbatim by another tooling level.


## Budget posture for `index_plane`
- correctness of `index_id`, `source_snapshot_set` is non-degradable
- lower-priority outputs such as lookup indices; reverse-reference indices degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.4-index-plane` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
