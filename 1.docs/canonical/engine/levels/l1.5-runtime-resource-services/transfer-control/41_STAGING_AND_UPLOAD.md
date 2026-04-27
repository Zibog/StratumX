# Staging and Upload

## Role

GPU/accelerator transfer staging.

## Data Model

Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §8.2


Staging slabs are reused, upload rings are budget-bounded, and upload visibility is explicit.
Transfer cost is overlap-friendly and may not become an unbounded frame-side spike machine.
Numeric ceilings are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`, `STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`, and `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`.

## Canonical Ceilings

| Item | Ceiling |
|---|---:|
| staging-backed memory | obey per-profile `staging-backed` ceiling from numeric source and memory constitution |
| upload bytes per presentation frame | <= 32 MiB p95 and <= 64 MiB p99 |
| transfer completion queue | <= 4,096 completion records per frame |

Raw source decode on the low-latency presentation path is illegal.
