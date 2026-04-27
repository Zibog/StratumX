# Core Contracts

## Role

Minimal cross-stack contract surface.

## Canonical Definition

`Core Contracts` is a canonical element of `engine_core` inside `L-1. Foundation`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Tiny contracts only: POD-like types, invariants, feature legality, error taxonomy hooks, and descriptor traits.
No convenience wrappers, runtime-owned branching policy, hidden allocation policy, or service ownership is legal here.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_core`:

- none

## Layer Links

- parent crate: `engine_core`
- level: `L-1. Foundation`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`
