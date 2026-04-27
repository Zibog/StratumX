# Test Surfaces

This contract belongs specifically to the budget runtime level and may not be reused verbatim by another tooling level.


## Required verification for `budget_runtime`
- invariant tests for `budget_scope_id`, `resource_class`, `hard_limit`
- dependency legality tests proving `{key}` resolves only `l6.8-cache-plane`
- communication tests covering pressure decisions; defer/deny signals for preview/build/release/runtime services
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.9-budget-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
