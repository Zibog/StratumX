# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| command_envelope_id | CommandEnvelopeId | stable envelope identity | unique per command submission |
| command_kind | CommandKind | closed command kind enum | must resolve through shared registry |
| target_ref_set | TargetRefSet | resolved public refs addressed by the command | must be explicit and bounded |
| issuer_session_id | ToolSessionId | originating tool session | must resolve through tool_session |
| mutation_intent_digest | MutationIntentDigest | digest of the payload for dedupe/audit | immutable after submission |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `command_envelopes` without consulting a hidden mirror.
