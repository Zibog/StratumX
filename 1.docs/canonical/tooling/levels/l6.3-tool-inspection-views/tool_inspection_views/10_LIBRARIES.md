# tool_inspection_views libraries

Allowed library focus:
- inspection-view schemas
- field grouping descriptors
- snapshot projection helpers

Library law:
helpers used by `tool_inspection_views` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_inspection_views` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_inspection_views` without inferring behavior from a neighboring sidecar.
