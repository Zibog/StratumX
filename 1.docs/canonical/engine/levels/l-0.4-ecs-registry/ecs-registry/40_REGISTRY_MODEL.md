# Registry Model

## Role

Structural ECS truth.

## Canonical Definition

`Registry Model` is a canonical element of `engine_ecs_registry` inside `L-0.4. ECS Registry`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Canonical registry of which entities and component classes structurally exist.
Registry owns structural shape only. It does not own steady-state traversal, convenience lookups, or compiled lane execution.
