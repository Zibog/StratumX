# Handle Types

## Role

Stable public handle families.

## Canonical Definition

`Handle Types` is a canonical element of `engine_handle` inside `L-0.8. Handle System`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Stable public handles are legal at crate boundaries.
Dense execution handles are internal traversal tokens only and may not escape compiled traversal plans or owning batches.
