# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.10-workspace-runtime`

## Allowed dependents or consumers
- activation rules
- assistant surface
- diagnostics and runtime bridge routing
- automation scripts that target surfaces

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_panel_refs` may depend only on the declared surfaces above because it exists solely to publish panel refs visible to tooling services while keeping actual layout and widget ownership in editor and must not become a backdoor for unrelated tooling state.
