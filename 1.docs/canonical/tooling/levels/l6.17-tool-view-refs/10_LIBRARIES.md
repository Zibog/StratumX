# Libraries

This contract is specific to `tool_view_refs` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- view-ref records
- view-kind enums
- hosting-surface descriptors
- view epoch comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_view_refs` beyond view refs for view-host coordination without taking ownership of rendered editor views
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_view_refs` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
