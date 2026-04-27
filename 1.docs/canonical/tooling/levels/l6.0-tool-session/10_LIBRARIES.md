# Libraries

This contract is specific to `tool_session` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- session identity records
- caller-scope enums
- attachment-scope cursors
- monotonic session-state transitions

## Forbidden library posture
- convenience wrappers that silently widen `tool_session` beyond public tool-session identity, lifecycle, and caller scope shared by all downstream sidecar traffic
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_session` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
