# Runtime Buffering

## Role

Acoustics runtime buffering.

## Data Model

Runtime buffering owns active voices, decode concurrency, critical-cue buffers, ambience buffers, and streaming page pressure inside the acoustics budget.
Numeric ceilings are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Buffer Matrix

| Item | Canonical law |
|---|---|
| active voices | obey numeric-source ceiling |
| decode concurrency | obey numeric-source ceiling |
| critical cue buffers | obey numeric-source ceiling |
| ambience buffers | obey numeric-source ceiling |

## Illegal Patterns

- blocking decode on a critical execution lane;
- unbounded ambience buffer growth;
- acoustics-owned residency governor outside runtime law.
