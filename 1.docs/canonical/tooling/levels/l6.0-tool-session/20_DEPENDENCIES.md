# Dependencies

## Legal dependencies
- `l6.0-authority-core`

## Allowed dependents or consumers
- all sidecar publication surfaces
- activation/runtime services that need caller scope
- audit/replay readers that correlate sidecar traffic by session

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_session` may depend only on the declared surfaces above because it exists solely to publish public tool-session identity, lifecycle, and caller scope shared by all downstream sidecar traffic and must not become a backdoor for unrelated tooling state.
