# tool_task_results dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.14-tool-task-requests`
- `l6.12-preview-runtime`
- `l6.11-validation-runtime`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

Dependency law:
`tool_task_results` may depend only on the listed surfaces because it publishes structured results of long-running tasks without owning their source truth.
