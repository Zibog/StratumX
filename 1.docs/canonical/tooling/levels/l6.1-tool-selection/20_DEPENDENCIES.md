# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.4-index-plane`

## Allowed dependents or consumers
- inspection views
- preview requests
- scene/content intents
- validation and assistant targeting

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_selection` may depend only on the declared surfaces above because it exists solely to publish published selection refs emitted by editor surfaces so tooling services can target the same objects without owning UI selection truth and must not become a backdoor for unrelated tooling state.
