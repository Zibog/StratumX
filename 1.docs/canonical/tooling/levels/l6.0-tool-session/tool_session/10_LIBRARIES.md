# tool_session libraries

Allowed library focus:
- session identity records
- caller-scope enums
- attachment-scope cursors

Library law:
helpers used by `tool_session` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_session` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_session` without inferring behavior from a neighboring sidecar.
