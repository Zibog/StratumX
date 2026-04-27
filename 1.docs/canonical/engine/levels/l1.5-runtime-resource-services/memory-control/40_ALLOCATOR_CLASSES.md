# Allocator Classes

## Role

Runtime allocation class model.

## Data Model

Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §6


Frame-scratch, tick-scratch, cell-scratch, session-persistent, streaming-resident, and staging-backed classes.
General allocator traffic is illegal in steady-state critical execution lanes.
Transient work uses owning arena or pool classes only.
Per-class ceilings are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` and `STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`.

## Per-Class Ceilings

| Class | `interactive-60` | `listen-host-60` | `headless-20` |
|---|---:|---:|---:|
| `frame-scratch` | 64 MiB | 64 MiB | 8 MiB |
| `tick-scratch` | 96 MiB | 128 MiB | 128 MiB |
| `cell-scratch` | 8 MiB per active chunk / 128 MiB aggregate | 8 MiB per active chunk / 160 MiB aggregate | 8 MiB per active chunk / 192 MiB aggregate |
| `session-persistent` | 384 MiB | 512 MiB | 512 MiB |
| `streaming-resident` | 1,280 MiB | 1,536 MiB | 1,152 MiB |
| `staging-backed` | 192 MiB | 256 MiB | 64 MiB |
