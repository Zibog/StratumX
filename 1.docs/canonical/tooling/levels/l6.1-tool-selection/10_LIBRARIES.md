# Libraries

This contract is specific to `tool_selection` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- selection-ref sets
- scope-normalization helpers
- selection epoch comparators
- stable target-ref codecs

## Forbidden library posture
- convenience wrappers that silently widen `tool_selection` beyond published selection refs emitted by editor surfaces so tooling services can target the same objects without owning UI selection truth
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_selection` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
