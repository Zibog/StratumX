# tool_preview_results libraries

Allowed library focus:
- preview result descriptors
- freshness comparators
- result-state enums

Library law:
helpers used by `tool_preview_results` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_preview_results` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_preview_results` without inferring behavior from a neighboring sidecar.
