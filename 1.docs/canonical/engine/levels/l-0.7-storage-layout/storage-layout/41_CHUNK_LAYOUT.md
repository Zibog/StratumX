# Chunk Layout

## Role

Chunked dense layout class.

## Canonical Definition

`Chunk Layout` is a canonical element of `engine_storage_layout` inside `L-0.7. Storage Layout`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Dense chunk layout groups hot signatures into partition-friendly contiguous blocks.
Chunk layout is legal only when the owning traversal plan freezes signature, access mode, and invalidation law.
Chunk layout may not redefine public identity or registry truth.
