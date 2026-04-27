# tool_activation_state libraries

Allowed library focus:
- activation state records
- resolved-rule links
- tool-mode enums

Library law:
helpers used by `tool_activation_state` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_activation_state` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_activation_state` without inferring behavior from a neighboring sidecar.
