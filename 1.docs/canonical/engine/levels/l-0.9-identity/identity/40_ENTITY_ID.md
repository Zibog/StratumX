# Entity ID

## Role

Authoritative entity identity token.

## Canonical Definition

`Entity ID` is a canonical element of `engine_identity` inside `L-0.9. Identity Primitives`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Stable entity identity token with generation-protected reuse.
Entity identity is never a dense traversal token and never carries execution-lane locality assumptions.
