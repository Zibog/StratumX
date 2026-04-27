# tool_diagnostics_events dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.11-validation-runtime`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

Dependency law:
`tool_diagnostics_events` may depend only on the listed surfaces because it publishes bounded diagnostics events emitted by validation, build, release, preview, and runtime services.
