# Selection And Focus Rules

This contract belongs specifically to the overlay and gizmo system editor level and is expected to become direct implementation work.


## Rules for `overlay_and_gizmo_system`
- selection and focus are explicit, publishable, and bounded for `overlay_set_id`, `gizmo_state_id`
- this level may own local focus presentation but may not fabricate hidden target mirrors
- pinned or multi-target modes must remain typed and auditable for `{key}` interactions

## Operational note
This file remains active and package-specific for `l8.6-overlay-and-gizmo-system` / `42_SELECTION_AND_FOCUS_RULES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 SELECTION AND FOCUS RULES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
