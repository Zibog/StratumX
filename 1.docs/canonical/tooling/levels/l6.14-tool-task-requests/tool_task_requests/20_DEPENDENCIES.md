# tool_task_requests dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.13-tool-activation-state`
- `l6.12-preview-runtime`
- `l6.11-validation-runtime`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

Dependency law:
`tool_task_requests` may depend only on the listed surfaces because it publishes long-running work requests that may end in build, preview, validation, release, or assistant tasks.
