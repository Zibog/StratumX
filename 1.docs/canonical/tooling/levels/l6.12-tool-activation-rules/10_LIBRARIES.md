# Libraries

This contract is specific to `tool_activation_rules` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- activation rule schemas
- required-surface descriptors
- deny-condition records
- priority comparators

## Forbidden library posture
- convenience wrappers that silently widen `tool_activation_rules` beyond tool activation rules, deny conditions, and prerequisite surfaces
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_activation_rules` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
