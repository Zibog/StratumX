# tool_scene_intents libraries

Allowed library focus:
- scene-intent envelopes
- target-entity-set codecs
- requested-effect enums

Library law:
helpers used by `tool_scene_intents` must stay publication-only and may not smuggle runtime, widget, layout, or mutation owners into the sidecar.

## Additional guardrail
`tool_scene_intents` may rely on typed helpers only when those helpers preserve publication-only semantics, explicit ownership boundaries, and replay-safe serialization.

## Review note
A reviewer should be able to identify why each helper exists for `tool_scene_intents` without inferring behavior from a neighboring sidecar.
