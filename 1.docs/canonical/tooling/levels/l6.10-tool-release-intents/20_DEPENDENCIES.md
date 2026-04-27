# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.13-tool-activation-state`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

## Allowed dependents or consumers
- build_runtime
- release_runtime
- automation meta flows
- reporting

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_release_intents` may depend only on the declared surfaces above because it exists solely to publish release-facing intents such as build channel selection, packaging class, or publish requests and must not become a backdoor for unrelated tooling state.
