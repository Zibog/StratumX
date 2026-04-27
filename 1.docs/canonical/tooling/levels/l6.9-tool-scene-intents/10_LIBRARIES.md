# Libraries

This contract is specific to `tool_scene_intents` and exists to prevent sidecar drift by analogy.

## Allowed library classes
- scene-intent envelopes
- target-entity-set codecs
- requested-effect enums
- world/scene scope descriptors

## Forbidden library posture
- convenience wrappers that silently widen `tool_scene_intents` beyond scene and world authoring intents before they are materialized as legal commands
- editor widget/layout implementations or runtime owners that belong elsewhere
- hidden caches, ad-hoc globals, or undeclared lower-package internals

## Audit rule
Every imported library for `tool_scene_intents` must preserve the exact sidecar ownership slice, invalidation rule, and publication discipline declared by this level.
