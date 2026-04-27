# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`

## Allowed dependents or consumers
- inspection views
- property editors
- assistant targeting
- runtime bridge routing

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_focus_refs` may depend only on the declared surfaces above because it exists solely to publish published focus refs for one active inspection or interaction target without owning widget focus behavior and must not become a backdoor for unrelated tooling state.
