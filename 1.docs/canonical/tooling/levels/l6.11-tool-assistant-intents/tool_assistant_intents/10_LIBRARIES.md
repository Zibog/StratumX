# tool_assistant_intents libraries

Allowed library focus:
- assistant-intent envelopes
- confidence-band enums
- target-scope descriptors

Library law:
helpers used by `tool_assistant_intents` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_assistant_intents` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_assistant_intents` without inferring behavior from a neighboring sidecar.
