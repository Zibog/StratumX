# Stale Handle Detection

## Role

Detection of stale references.

## Canonical Definition

`Stale Handle Detection` is a canonical element of `engine_handle` inside `L-0.8. Handle System`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Generation-aware stale detection logic that rejects references after identity turnover.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_handle`:

- `engine_core`
- `engine_identity`

## Layer Links

- parent crate: `engine_handle`
- level: `L-0.8. Handle System`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
