# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.6-tool-diagnostics-events`
- `l6.5-derived-plane`

## Allowed dependents or consumers
- editor diagnostics surface
- runtime inspector overlays
- assistant summaries
- reporting

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_diagnostics_views` may depend only on the declared surfaces above because it exists solely to publish filtered and grouped diagnostics views for consumers that need stable issue presentations and must not become a backdoor for unrelated tooling state.
