# Libraries

This contract is specific to `tool_content_intents` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- content-intent envelopes
- target-asset-set codecs
- requested-effect enums
- issuer-scope descriptors

## Forbidden library posture
- convenience wrappers that silently widen `tool_content_intents` beyond content-authoring intents that later become commands, imports, or build jobs
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_content_intents` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
