# tool_diagnostics_views libraries

Allowed library focus:
- diagnostic view schemas
- grouping/filter digests
- issue-set refs

Library law:
helpers used by `tool_diagnostics_views` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_diagnostics_views` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_diagnostics_views` without inferring behavior from a neighboring sidecar.
