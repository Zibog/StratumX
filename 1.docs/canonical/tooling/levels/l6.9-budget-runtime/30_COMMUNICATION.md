# Communication

This communication contract belongs specifically to the budget runtime tooling level.

## Sends or publishes
- pressure decisions
- defer/deny signals for preview/build/release/runtime services
- sender and receiver scopes are explicit

## Required posture
- sender and receiver scopes are explicit for budget runtime work
- published records such as budget_scope_id, resource_class remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for budget runtime may not be copied to another level without changing operational meaning.
