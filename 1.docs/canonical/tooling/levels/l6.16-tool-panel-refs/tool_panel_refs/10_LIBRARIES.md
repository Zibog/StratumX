# tool_panel_refs libraries

Allowed library focus:
- panel-ref records
- panel-kind enums
- hosting-surface descriptors

Library law:
helpers used by `tool_panel_refs` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_panel_refs` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_panel_refs` without inferring behavior from a neighboring sidecar.
