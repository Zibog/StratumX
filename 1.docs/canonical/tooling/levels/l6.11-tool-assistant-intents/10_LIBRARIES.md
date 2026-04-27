# Libraries

This contract is specific to `tool_assistant_intents` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- assistant-intent envelopes
- confidence-band enums
- target-scope descriptors
- proposal provenance records

## Forbidden library posture
- convenience wrappers that silently widen `tool_assistant_intents` beyond assistant-originated suggestions or non-authoritative intents before they are lowered into legal commands
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_assistant_intents` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
