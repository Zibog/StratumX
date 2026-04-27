# Tool Assistant Intents

## Role
`tool_assistant_intents` carries assistant-originated suggestions or non-authoritative intents before they are lowered into legal commands.

## Owns
- `assistant_intent_id`
- `intent_kind`
- `target_scope`
- `confidence_band`
- `issuer_session_id`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_assistant_intents` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
