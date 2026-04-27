# Coordinates

## Role

World coordinate model.

## Canonical Definition

`Coordinates` is a canonical element of `engine_world_spatial` inside `L-0.1. World Spatial`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Coordinate spaces and position semantics for authoritative world-space reasoning.
Canonical coordinate law supports world-local, region-local, and rebased presentation coordinates.
Far-distance addressing may use quantized or fixed-step addressing to avoid precision collapse.
