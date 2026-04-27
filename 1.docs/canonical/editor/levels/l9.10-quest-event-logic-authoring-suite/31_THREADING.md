# Threading

This contract belongs specifically to the quest event logic authoring suite editor level and is expected to become direct implementation work.


## Threading posture for `quest_event_logic_authoring_suite`
- interactive ownership of `logic_suite_session_id`, `quest_graph_ref` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to quest/event graph edit requests; signal track updates publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l9.10-quest-event-logic-authoring-suite` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
