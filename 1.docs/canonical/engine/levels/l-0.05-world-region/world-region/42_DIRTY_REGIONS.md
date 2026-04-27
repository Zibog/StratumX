# Dirty Regions

## Role

Region and chunk dirtiness law.

## Canonical Definition

`Dirty Regions` is a canonical element of `engine_world_region` inside `L-0.05. World Region`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Dirty propagation is width-bounded and versioned.
Dirty state may rise from chunk to region or region to replication/export, but may not force whole-world publication when local propagation is sufficient.
