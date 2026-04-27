# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.13-tool-activation-state`
- `l6.12-preview-runtime`
- `l6.11-validation-runtime`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

## Allowed dependents or consumers
- preview/build/release/validation runtimes
- automation/batch services
- reporting

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_task_requests` may depend only on the declared surfaces above because it exists solely to publish long-running work requests that may end in build, preview, validation, release, or assistant tasks and must not become a backdoor for unrelated tooling state.
