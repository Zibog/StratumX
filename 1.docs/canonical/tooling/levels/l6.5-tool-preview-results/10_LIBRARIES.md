# Libraries

This contract is specific to `tool_preview_results` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- preview result descriptors
- freshness comparators
- result-state enums
- preview artifact ref wrappers

## Forbidden library posture
- convenience wrappers that silently widen `tool_preview_results` beyond preview result refs and status summaries returned by preview_runtime
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_preview_results` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
