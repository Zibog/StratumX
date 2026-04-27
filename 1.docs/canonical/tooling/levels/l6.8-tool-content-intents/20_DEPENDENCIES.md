# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`
- `l6.13-tool-activation-state`

## Allowed dependents or consumers
- build/import services
- assistant lowering
- automation and batch services

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_content_intents` may depend only on the declared surfaces above because it exists solely to publish content-authoring intents that later become commands, imports, or build jobs and must not become a backdoor for unrelated tooling state.
