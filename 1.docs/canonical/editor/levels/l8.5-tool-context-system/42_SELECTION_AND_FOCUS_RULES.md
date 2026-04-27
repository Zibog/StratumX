# Selection And Focus Rules

This contract belongs specifically to the tool context system editor level and is expected to become direct implementation work.


## Rules for `tool_context_system`
- selection and focus are explicit, publishable, and bounded for `tool_context_id`, `active_tool_kind`
- this level may own local focus presentation but may not fabricate hidden target mirrors
- pinned or multi-target modes must remain typed and auditable for `{key}` interactions

## Operational note
This file remains active and package-specific for `l8.5-tool-context-system` / `42_SELECTION_AND_FOCUS_RULES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 SELECTION AND FOCUS RULES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
