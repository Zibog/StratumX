# Selection And Focus Rules

This contract belongs specifically to the graph authoring service editor level and is expected to become direct implementation work.


## Rules for `graph_authoring_service`
- selection and focus are explicit, publishable, and bounded for `graph_editor_session_id`, `graph_target_ref`
- this level may own local focus presentation but may not fabricate hidden target mirrors
- pinned or multi-target modes must remain typed and auditable for `{key}` interactions

## Operational note
This file remains active and package-specific for `l10.2-graph-authoring-service` / `42_SELECTION_AND_FOCUS_RULES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 SELECTION AND FOCUS RULES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
