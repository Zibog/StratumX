# Tool Activation State

## Role
`tool_activation_state` publishes the active activation state for tools and modes after rules are evaluated.

## Owns
- `activation_state_id`
- `tool_kind`
- `state`
- `resolved_rule_id`
- `state_epoch`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_activation_state` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
