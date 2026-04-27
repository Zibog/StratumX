# Apply Payloads

## Role

Authoritative apply batches.

## Canonical Definition

`Apply Payloads` is a canonical element of `engine_storage_mutation` inside `L-0.5. Storage Mutation`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Apply payloads are ordered, family-tagged, region-tagged batches that feed segmented apply.
They may not require whole-world monolithic integration when regional or island-scoped integration is legal.
