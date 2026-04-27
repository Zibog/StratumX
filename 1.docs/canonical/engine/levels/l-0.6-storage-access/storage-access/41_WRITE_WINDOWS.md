# Write Windows

## Role

Controlled mutable windows.

## Canonical Definition

`Write Windows` is a canonical element of `engine_storage_access` inside `L-0.6. Storage Access`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Time-bounded and legality-bounded mutable access windows opened above storage layout.
Every write window is bound to an access descriptor, traversal plan legality, and runtime scheduling legality.
Unregistered mutable bypasses are illegal.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_storage_access`:

- `engine_core`
- `engine_handle`
- `engine_storage_layout`
