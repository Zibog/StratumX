# Validation Model

## Role

Handle validation law.

## Canonical Definition

`Validation Model` is a canonical element of `engine_handle` inside `L-0.8. Handle System`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Stable public handles validate through generation-aware checks.
Validation is legal on boundary entry, diagnostics, and plan-build boundaries.
Repeated stable-handle validation inside steady-state compiled traversal is illegal.
