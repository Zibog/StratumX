# Budget Envelope

This contract belongs specifically to the tool panel refs level and may not be reused verbatim by another tooling level.


## Budget posture for `tool_panel_refs`
- correctness of `panel_ref_id`, `panel_kind` is non-degradable
- lower-priority outputs such as published `tool_panel_refs` rows for downstream services; bounded status or routing updates where applicable degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.16-tool-panel-refs` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
