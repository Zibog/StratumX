# Selection And Focus Rules

This contract belongs specifically to the outliner system editor level and is expected to become direct implementation work.


## Rules for `outliner_system`
- selection and focus are explicit, publishable, and bounded for `outliner_view_id`, `hierarchy_projection_ref`
- this level may own local focus presentation but may not fabricate hidden target mirrors
- pinned or multi-target modes must remain typed and auditable for `{key}` interactions

## Operational note
This file remains active and package-specific for `l8.2-outliner-system` / `42_SELECTION_AND_FOCUS_RULES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 SELECTION AND FOCUS RULES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
