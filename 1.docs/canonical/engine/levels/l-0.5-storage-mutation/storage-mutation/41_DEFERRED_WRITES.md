# Deferred Writes

## Role

Deferred write model.

## Canonical Definition

`Deferred Writes` is a canonical element of `engine_storage_mutation` inside `L-0.5. Storage Mutation`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Write deferral structures that keep direct world mutation out of compute phases.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_storage_mutation`:

- `engine_core`
- `engine_handle`
- `engine_storage_layout`
- `engine_storage_access`

## Layer Links

- parent crate: `engine_storage_mutation`
- level: `L-0.5. Storage Mutation`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
