# tool_task_results libraries

Allowed library focus:
- task result envelopes
- result-state enums
- artifact/result refs

Library law:
helpers used by `tool_task_results` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_task_results` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_task_results` without inferring behavior from a neighboring sidecar.
