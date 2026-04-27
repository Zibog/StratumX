# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.12-tool-activation-rules`
- `l6.10-workspace-runtime`

## Allowed dependents or consumers
- editor tool context
- preview/build/release request gating
- assistant suggestions
- automation

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_activation_state` may depend only on the declared surfaces above because it exists solely to publish active activation state for tools and modes after activation rules are evaluated and must not become a backdoor for unrelated tooling state.
