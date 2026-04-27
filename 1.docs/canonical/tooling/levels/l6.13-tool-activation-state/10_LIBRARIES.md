# Libraries

This contract is specific to `tool_activation_state` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- activation state records
- resolved-rule links
- tool-mode enums
- epoch comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_activation_state` beyond active activation state for tools and modes after activation rules are evaluated
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_activation_state` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
