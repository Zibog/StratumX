# Tool Activation Rules

## Role
`tool_activation_rules` declares tool activation rules, deny conditions, and prerequisite surfaces.

## Owns
- `activation_rule_id`
- `tool_kind`
- `required_surface_set`
- `deny_condition_set`
- `priority`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_activation_rules` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
