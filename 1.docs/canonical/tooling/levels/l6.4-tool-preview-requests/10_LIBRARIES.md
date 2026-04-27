# Libraries

This contract is specific to `tool_preview_requests` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- preview request envelopes
- quality-hint descriptors
- cancel-token records
- preview target-set codecs

## Forbidden library posture
- convenience wrappers that silently widen `tool_preview_requests` beyond preview requests flowing from editor/tools into preview_runtime
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_preview_requests` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
