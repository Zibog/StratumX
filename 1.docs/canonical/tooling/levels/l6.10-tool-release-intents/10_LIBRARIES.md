# Libraries

This contract is specific to `tool_release_intents` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- release-intent envelopes
- channel/package enums
- artifact-scope descriptors
- issuer provenance records

## Forbidden library posture
- convenience wrappers that silently widen `tool_release_intents` beyond release-facing intents such as build channel selection, packaging class, or publish requests
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_release_intents` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
