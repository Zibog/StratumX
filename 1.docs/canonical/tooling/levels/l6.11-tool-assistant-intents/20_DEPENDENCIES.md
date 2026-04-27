# Dependencies

## Legal dependencies
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6a.2-proposal-runtime`
- `l6a.3-lowering-runtime`
- `l6a.4-safety-gates`

## Allowed dependents or consumers
- assistant lowering runtime
- automation/batch services
- editor assistant surface
- audit/replay

## Forbidden dependencies
- undeclared editor-local widget/layout truth
- hidden shadow stores outside declared planes, runtimes, or sidecars
- adjacent semantic classes not named above

## Dependency law
`tool_assistant_intents` may depend only on the declared surfaces above because it exists solely to publish assistant-originated suggestions or non-authoritative intents before they are lowered into legal commands and must not become a backdoor for unrelated tooling state.
