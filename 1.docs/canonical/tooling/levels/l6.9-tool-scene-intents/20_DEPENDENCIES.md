# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`
- `l6.13-tool-activation-state`

## Allowed dependents or consumers
- scene/world authoring services
- runtime bridge
- automation
- assistant lowering

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_scene_intents` may depend only on the declared surfaces above because it exists solely to publish scene and world authoring intents before they are materialized as legal commands and must not become a backdoor for unrelated tooling state.
