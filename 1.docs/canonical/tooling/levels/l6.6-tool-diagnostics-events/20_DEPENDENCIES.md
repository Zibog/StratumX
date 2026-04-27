# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.11-validation-runtime`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

## Allowed dependents or consumers
- diagnostics views
- editor diagnostics surface
- assistant explanation and auto-fix suggestions
- audit/reporting

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_diagnostics_events` may depend only on the declared surfaces above because it exists solely to publish bounded diagnostics events emitted by validation, build, release, preview, and runtime services and must not become a backdoor for unrelated tooling state.
