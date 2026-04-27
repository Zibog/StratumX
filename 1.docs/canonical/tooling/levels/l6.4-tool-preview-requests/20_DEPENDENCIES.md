# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`
- `l6.12-preview-runtime`

## Allowed dependents or consumers
- preview_runtime
- task-request lowering
- diagnostics correlation readers

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_preview_requests` may depend only on the declared surfaces above because it exists solely to publish preview requests flowing from editor/tools into preview_runtime and must not become a backdoor for unrelated tooling state.
