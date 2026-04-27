# Locality Model

## Role

Locality rules.

## Canonical Definition

`Locality Model` is a canonical element of `engine_storage_layout` inside `L-0.7. Storage Layout`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Cache locality, spatial locality, partition locality, and traversal-lane locality classes that shape layout decisions.
Prepared dense traversal is legal for critical execution lanes only when locality class is explicit and query-plan invalidation is frozen.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_storage_layout`:

- `engine_core`
