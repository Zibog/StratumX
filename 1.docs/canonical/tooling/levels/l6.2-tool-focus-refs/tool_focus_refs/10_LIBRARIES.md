# tool_focus_refs libraries

Allowed library focus:
- focus-ref records
- focus mode enums
- surface binding descriptors

Library law:
helpers used by `tool_focus_refs` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_focus_refs` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_focus_refs` without inferring behavior from a neighboring sidecar.
