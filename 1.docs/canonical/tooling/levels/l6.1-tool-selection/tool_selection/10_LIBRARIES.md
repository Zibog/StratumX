# tool_selection libraries

Allowed library focus:
- selection-ref sets
- scope-normalization helpers
- selection epoch comparators

Library law:
helpers used by `tool_selection` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_selection` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_selection` without inferring behavior from a neighboring sidecar.
