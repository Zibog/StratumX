# Tool Content Intents

## Role
`tool_content_intents` carries content-authoring intents that later become commands or build jobs.

## Owns
- `content_intent_id`
- `intent_kind`
- `target_asset_set`
- `requested_effect`
- `issuer_session_id`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_content_intents` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
