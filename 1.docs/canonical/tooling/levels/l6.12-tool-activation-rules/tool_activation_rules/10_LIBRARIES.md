# tool_activation_rules libraries

Allowed library focus:
- activation rule schemas
- required-surface descriptors
- deny-condition records

Library law:
helpers used by `tool_activation_rules` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_activation_rules` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_activation_rules` without inferring behavior from a neighboring sidecar.
