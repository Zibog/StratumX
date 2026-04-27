# Spatial Addressing

## Role

Deterministic spatial addressing.

## Canonical Definition

`Spatial Addressing` is a canonical element of `engine_world_spatial` inside `L-0.1. World Spatial`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Spatial addressing binds world-local coordinates to region and chunk ownership.
Addressing must remain deterministic across rebasing and may not require presentation-space floating precision for authoritative region ownership.
