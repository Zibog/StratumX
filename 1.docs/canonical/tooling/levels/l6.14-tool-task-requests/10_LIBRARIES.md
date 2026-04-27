# Libraries

This contract is specific to `tool_task_requests` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- task request envelopes
- priority/scheduling enums
- target-scope descriptors
- issuer provenance records

## Forbidden library posture
- convenience wrappers that silently widen `tool_task_requests` beyond long-running work requests that may end in build, preview, validation, release, or assistant tasks
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_task_requests` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
