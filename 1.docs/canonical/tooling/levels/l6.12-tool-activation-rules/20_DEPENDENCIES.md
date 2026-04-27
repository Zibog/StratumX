# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.10-workspace-runtime`
- `l6.16-tool-panel-refs`
- `l6.17-tool-view-refs`

## Allowed dependents or consumers
- tool_activation_state
- workspace_runtime
- editor tool routing
- assistant surface

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_activation_rules` may depend only on the declared surfaces above because it exists solely to publish tool activation rules, deny conditions, and prerequisite surfaces and must not become a backdoor for unrelated tooling state.
