# tool_diagnostics_events libraries

Allowed library focus:
- diagnostic event envelopes
- severity enums
- source-scope descriptors

Library law:
helpers used by `tool_diagnostics_events` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_diagnostics_events` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_diagnostics_events` without inferring behavior from a neighboring sidecar.
