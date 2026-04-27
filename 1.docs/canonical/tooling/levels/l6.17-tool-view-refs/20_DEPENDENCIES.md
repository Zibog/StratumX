# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.10-workspace-runtime`
- `l6.16-tool-panel-refs`

## Allowed dependents or consumers
- activation rules
- assistant and diagnostics overlays
- runtime bridge and preview routing
- automation scripts

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_view_refs` may depend only on the declared surfaces above because it exists solely to publish view refs for view-host coordination without taking ownership of rendered editor views and must not become a backdoor for unrelated tooling state.
