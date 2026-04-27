# Feature Flags

## Role

Feature legality at the foundation boundary.

## Canonical Definition

`Feature Flags` is a canonical element of `engine_core` inside `L-1. Foundation`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Feature flags select legal capability sets only.
They may not smuggle allocator policy, runtime ownership, or scene-policy shortcuts into lower stack contracts.
Flags that materially affect execution law, memory law, replay law, or startup legality require explicit canonical documentation.
