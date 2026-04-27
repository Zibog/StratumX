# Mutation Buffers

## Role

Batched mutation staging.

## Canonical Definition

`Mutation Buffers` is a canonical element of `engine_storage_mutation` inside `L-0.5. Storage Mutation`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Temporary mutation carriers that accumulate storage-side changes before authoritative apply.
Mutation buffers must support merge/coalesce rules, idempotence classes, batch ordering, and tombstone/compaction windows so apply cost scales by batches instead of random micro-writes.
