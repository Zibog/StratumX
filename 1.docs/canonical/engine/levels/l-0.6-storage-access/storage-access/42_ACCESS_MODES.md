# Access Modes

## Role

Access legality model.

## Canonical Definition

`Access Modes` is a canonical element of `engine_storage_access` inside `L-0.6. Storage Access`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Mode descriptors that define whether storage access is read, write, staged, or mixed under explicit constraints.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_storage_access`:

- `engine_core`
- `engine_handle`
- `engine_storage_layout`

## Layer Links

- parent crate: `engine_storage_access`
- level: `L-0.6. Storage Access`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
