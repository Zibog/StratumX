# Libraries

This contract is specific to `tool_diagnostics_views` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- diagnostic view schemas
- grouping/filter digests
- issue-set refs
- freshness comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_diagnostics_views` beyond filtered and grouped diagnostics views for consumers that need stable issue presentations
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_diagnostics_views` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
