# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.14-tool-task-requests`
- `l6.12-preview-runtime`
- `l6.11-validation-runtime`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

## Allowed dependents or consumers
- editor task/result surfaces
- diagnostics views
- reporting
- assistant follow-up actions

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_task_results` may depend only on the declared surfaces above because it exists solely to publish structured results of long-running tasks without owning their source truth and must not become a backdoor for unrelated tooling state.
