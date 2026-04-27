# Snapshots

## Role

Readable authoritative world snapshots.

## Canonical Definition

`Snapshots` is a canonical element of `engine_world` inside `L0. World Truth`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Canonical snapshots are immutable read products for runtime, critical families, sync, and synthesis.
Snapshots also serve deterministic replay and divergence checkpoints and therefore remain versioned, ordered, and profile-visible.
