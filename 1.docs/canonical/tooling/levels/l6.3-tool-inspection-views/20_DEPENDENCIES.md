# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.2-tool-focus-refs`
- `l6.3-snapshot-plane`
- `l6.5-derived-plane`

## Allowed dependents or consumers
- editor inspector
- diagnostics surface
- assistant explanation panels
- runtime comparison tools

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_inspection_views` may depend only on the declared surfaces above because it exists solely to publish structured inspection views generated from snapshots for inspectors, diagnostics, and property review surfaces and must not become a backdoor for unrelated tooling state.
