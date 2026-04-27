# Lookup Model

## Role

Parallel-safe material lookup.

## Data Model

Lookup is read-heavy, compact-id based, flattened, and cache-friendly.
Descriptor chasing through long ownership chains is illegal in critical execution lanes.
Descriptor width and reaction-row width become authoritative only when frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Lookup Class Matrix

| Class | Canonical law | Cache posture | Fallback/miss posture | Illegal posture |
|---|---|---|---|---|
| descriptor row | compact immutable id | bounded hot descriptor cache | one deterministic fallback step | dynamic owner-chain walk |
| reaction row | flattened immutable row | bounded hot reaction cache | one deterministic authored default reaction | nested descriptor chase |
| cache ledger | diagnostics-law bounded only | no hidden mutable global cache | miss counts bounded and optional | permanent hot-path miss ledger |
| family consumers | kinetics/field/acoustics/imaging read only | no ownership transfer | deterministic fallback only | consumer rewrites lookup law |

## Binding Table

| Binding | Canonical source |
|---|---|
| shared hard numbers | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| diagnostics miss posture | `STRATUMX_DIAGNOSTICS_CEILING_LAW.md` |
| consumer families | kinetics, field, acoustics, imaging local docs |

## Fallback Matrix

| Fallback class | Canonical posture | Illegal posture |
|---|---|---|
| descriptor miss | one deterministic fallback step | recursive owner-chain walk |
| reaction miss | deterministic authored default | dynamic query chase |
| cache miss ledger | optional diagnostics-only | persistent hot-path ledger |

## Local Operating Law

Lookup legality is defined by lookup class, cache class, fallback class, and miss posture together.
Any local contract that reintroduces dynamic ownership walks is illegal.
