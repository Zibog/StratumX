# tool_release_intents libraries

Allowed library focus:
- release-intent envelopes
- channel/package enums
- artifact-scope descriptors

Library law:
helpers used by `tool_release_intents` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_release_intents` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_release_intents` without inferring behavior from a neighboring sidecar.
