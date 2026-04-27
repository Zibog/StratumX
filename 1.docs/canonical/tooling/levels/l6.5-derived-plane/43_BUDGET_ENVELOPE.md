# Budget Envelope

This contract belongs specifically to the derived plane level and may not be reused verbatim by another tooling level.


## Budget posture for `derived_plane`
- correctness of `derived_projection_id`, `source_snapshot_set` is non-degradable
- lower-priority outputs such as derived projections; heatmaps, summaries, and prepared views for editor/runtime surfaces degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.5-derived-plane` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
