# Libraries

This contract is specific to `tool_panel_refs` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- panel-ref records
- panel-kind enums
- hosting-surface descriptors
- epoch comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_panel_refs` beyond panel refs visible to tooling services while keeping actual layout and widget ownership in editor
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_panel_refs` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
