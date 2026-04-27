# Visibility

## Role

Imaging visibility hierarchy.

## Canonical Definition

`Visibility` is a canonical element of `engine_imaging` inside `L3.1. Synthesis Systems`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Visibility is hierarchical and bounded. Numeric ceilings are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Minimum Hierarchy Shape

| Layer | Canonical law |
|---|---|
| region or cell visibility | mandatory |
| instance-cluster visibility | mandatory and bounded by numeric-source cluster size/payload ceilings |
| occlusion stage boundary | explicit and bounded by numeric-source feedback lag |
| submission bucket set | bounded by numeric-source bucket ceiling |

## Canonical Law

- visibility results may be at most one frame stale;
- visibility may not own world truth or residency ownership;
- culling granularity and cluster sizes may not exceed numeric-source ceilings.
