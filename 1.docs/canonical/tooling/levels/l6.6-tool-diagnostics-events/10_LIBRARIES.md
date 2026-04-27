# Libraries

This contract is specific to `tool_diagnostics_events` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- diagnostic event envelopes
- severity enums
- source-scope descriptors
- ordering-cursor helpers

## Forbidden library posture
- convenience wrappers that silently widen `tool_diagnostics_events` beyond bounded diagnostics events emitted by validation, build, release, preview, and runtime services
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_diagnostics_events` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
