# Libraries

This contract is specific to `tool_focus_refs` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- focus-ref records
- focus mode enums
- surface binding descriptors
- epoch comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_focus_refs` beyond published focus refs for one active inspection or interaction target without owning widget focus behavior
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_focus_refs` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
