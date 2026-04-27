# Tool Release Intents

## Role
`tool_release_intents` carries release-facing intents such as build channel selection, packaging class, or publish requests.

## Owns
- `release_intent_id`
- `intent_kind`
- `target_channel`
- `artifact_scope`
- `issuer_session_id`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_release_intents` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
