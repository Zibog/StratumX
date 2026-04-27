# Libraries

This contract is specific to `tool_inspection_views` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- inspection-view schemas
- field grouping descriptors
- snapshot projection helpers
- view generation cursors

## Forbidden library posture
- convenience wrappers that silently widen `tool_inspection_views` beyond structured inspection views generated from snapshots for inspectors, diagnostics, and property review surfaces
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_inspection_views` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
