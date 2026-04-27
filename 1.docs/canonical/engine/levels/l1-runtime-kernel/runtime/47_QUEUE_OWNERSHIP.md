# Queue Ownership

## Role

Runtime-owned queue discipline.

## Data Model

Runtime owns queue creation, queue depth, queue age, and flush deadlines.
No crate may create hidden shadow queues for convenience.
Authoritative queue numbers are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Queue Classes

| Queue | Depth ceiling | Age or flush ceiling |
|---|---:|---:|
| apply queue | <= 16,384 records per segment and <= 65,536 aggregate | <= 1 authoritative tick |
| transfer completion queue | <= 4,096 records | <= 2 frames or <= 2 ticks |
| per-connection publication queue | <= 1,024 records and <= 256 KiB queued payload per peer | control <= 100 ms; bulk <= 500 ms |
| low-latency presentable-frame queue | <= 1 frame | <= 1 frame |

## Illegal Patterns

- hidden family-local queues that outlive runtime visibility;
- tiny depth with unbounded queue age;
- queue spillover into unrelated allocators without runtime ownership.
