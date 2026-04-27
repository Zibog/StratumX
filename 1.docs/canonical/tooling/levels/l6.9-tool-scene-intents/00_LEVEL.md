# Tool Scene Intents

## Role
`tool_scene_intents` carries scene/world authoring intents before they are materialized as commands.

## Owns
- `scene_intent_id`
- `intent_kind`
- `target_entity_set`
- `requested_effect`
- `issuer_session_id`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_scene_intents` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
