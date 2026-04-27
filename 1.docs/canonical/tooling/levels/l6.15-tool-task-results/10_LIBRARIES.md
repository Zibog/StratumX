# Libraries

This contract is specific to `tool_task_results` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- task result envelopes
- result-state enums
- artifact/result refs
- completion cursor comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_task_results` beyond structured results of long-running tasks without owning their source truth
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_task_results` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
