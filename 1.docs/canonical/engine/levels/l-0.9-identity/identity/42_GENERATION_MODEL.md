# Generation Model

## Role

Identity reuse legality.

## Canonical Definition

`Generation Model` is a canonical element of `engine_identity` inside `L-0.9. Identity Primitives`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Identity reuse is generation-protected and reuse-delayed.
Generation width must be sufficient to make stale-handle aliasing non-trivial under canonical stress tests, and free-list reuse may not immediately recycle freshly retired identities into the same execution epoch.
