# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.12-preview-runtime`
- `l6.4-tool-preview-requests`

## Allowed dependents or consumers
- editor preview surfaces
- diagnostics and validation cross-checks
- assistant explanation views

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_preview_results` may depend only on the declared surfaces above because it exists solely to publish preview result refs and status summaries returned by preview_runtime and must not become a backdoor for unrelated tooling state.
