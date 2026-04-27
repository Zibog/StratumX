# tool_task_requests libraries

Allowed library focus:
- task request envelopes
- priority/scheduling enums
- target-scope descriptors

Library law:
helpers used by `tool_task_requests` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_task_requests` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_task_requests` without inferring behavior from a neighboring sidecar.
