# Apply Boundary

## Role

Controlled world integration boundary.

## Canonical Definition

`Apply Boundary` is a canonical element of `engine_world` inside `L0. World Truth`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Single authoritative boundary where staged changes become world truth.
Apply is segmented: region-scoped, island-scoped, family-tagged, and publication-safe.
Whole-world monolithic integration is illegal when a legal segmented apply exists.
Numeric ceilings are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Segmentation Ceilings

| Item | Ceiling |
|---|---:|
| max segments per authoritative tick | <= 256 |
| max family fan-out per segment | <= 8 |
| publish-order passes across all segments | <= 2 ordered passes |

## Illegal Patterns

- recursive segment spawning inside apply;
- publication order that depends on hash-map iteration or worker race;
- micro-segmentation used to hide monolithic apply cost.
