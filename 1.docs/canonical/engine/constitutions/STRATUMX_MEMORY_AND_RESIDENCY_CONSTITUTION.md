# STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION

## 1. Purpose

This document freezes allocator classes, residency classes, hysteresis windows, pressure-response law, per-class byte ceilings, VRAM partitions, and host-vs-client memory splits.
It turns memory-control into an enforcing governor, not a naming layer.
Numeric constants are authored in `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## 2. Canonical Allocator Classes

- `frame-scratch`: reset every frame, never crosses frame boundary;
- `tick-scratch`: reset every authoritative tick, may not cross apply boundary;
- `cell-scratch`: region or chunk bounded temporary work;
- `session-persistent`: session-long metadata and caches;
- `streaming-resident`: stream-owned resource backing;
- `staging-backed`: upload or transfer-facing memory tied to fence release.

## 3. Canonical Residency Classes

- `hot`: immediately needed in active execution or presentation window;
- `warm`: likely needed soon but not immediately;
- `cold`: legally evictable;
- `quarantined`: pending fence or completion visibility.

## 4. Per-Class Byte Ceilings

### 4.1. `interactive-60`

| Class | Ceiling |
|---|---:|
| `frame-scratch` | <= 64 MiB |
| `tick-scratch` | <= 96 MiB |
| `cell-scratch` | <= 8 MiB per active chunk and <= 128 MiB aggregate |
| `session-persistent` | <= 384 MiB |
| `streaming-resident` | <= 1,280 MiB |
| `staging-backed` | <= 192 MiB |

### 4.2. `listen-host-60`

| Class | Ceiling |
|---|---:|
| `frame-scratch` | <= 64 MiB |
| `tick-scratch` | <= 128 MiB |
| `cell-scratch` | <= 8 MiB per active chunk and <= 160 MiB aggregate |
| `session-persistent` | <= 512 MiB |
| `streaming-resident` | <= 1,536 MiB |
| `staging-backed` | <= 256 MiB |

### 4.3. `headless-20`

| Class | Ceiling |
|---|---:|
| `frame-scratch` | <= 8 MiB |
| `tick-scratch` | <= 128 MiB |
| `cell-scratch` | <= 8 MiB per active chunk and <= 192 MiB aggregate |
| `session-persistent` | <= 512 MiB |
| `streaming-resident` | <= 1,152 MiB |
| `staging-backed` | <= 64 MiB |

These ceilings live inside the profile-wide RAM hot/warm ceilings frozen by `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`.

## 5. VRAM Class Partition

### 5.1. `interactive-60` and `listen-host-60`

| VRAM class | Ceiling |
|---|---:|
| textures hot set | <= 1,024 MiB |
| geometry buffers hot set | <= 384 MiB |
| render targets persistent | <= 384 MiB |
| transient render targets | <= 192 MiB |
| pipeline, descriptor, and shader-side state | <= 64 MiB |

### 5.2. Rule

No single graphics class may borrow silently from another class. Temporary overflow requires explicit degradation or frame-budget failure, not hidden budget theft.

## 6. Hysteresis Law

Promotion, demotion, promotion debt, and eviction batch ceilings are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.
Promotion and demotion may not oscillate per frame or per tick.

## 7. Pressure Law

- pressure must be explicit, observable, and service-owned;
- pressure response ladder is: trim optional warm caches -> deny optional prefetch -> delay optional uploads -> evict cold -> demote warm -> fail illegal request;
- mandatory response latency after breach: <= 2 realtime frames or <= 2 headless ticks;
- pressure response may not mutate world truth or break deterministic replay.

## 8. Host-vs-Client Memory Split

On `listen-host-60`, session-persistent, delivery-state, and history-state memory must reserve at least 40% for local presentation/runtime ownership and may allocate at most 60% to host-side remote session burden.
No single remote peer may consume more than 20% of the host-side share.

## 9. Allocator Law

- general-purpose allocation is illegal in steady-state critical execution lanes;
- transient work uses owning arenas or pool classes only;
- security-hardened or debug allocators are legal outside strict steady-state lanes;
- pressure signals must identify which allocator class crossed which ceiling.
