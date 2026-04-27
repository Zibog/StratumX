# L6 Plane Separation Proof v4

## Purpose

This document proves that L6 is not a monolithic layer but a set of narrow, separated planes with distinct responsibilities.

## L6 Plane Taxonomy

### Authority/Data Plane Separation

| Plane | Responsibility | Owns Truth | Mutable | Notes |
|-------|----------------|------------|---------|-------|
| L6.0 Authority Core | Command routing, session management | No | Yes | Authority only, not data |
| L6.2 Transaction Ledger | Transaction records | Yes | Append-only | Immutable history |
| L6.3 Snapshot Plane | Versioned snapshots | No | Immutable | Read-only views |
| L6.4 Index Plane | Derived indexes | No | Rebuildable | Disposable derivations |
| L6.5 Derived Plane | Derived projections | No | Rebuildable | Disposable derivations |
| L6.6 Artifact Plane | Build artifacts | No | Immutable | Deterministic outputs |
| L6.7 Stream Plane | Event streams | No | Append-only | Bounded streams |
| L6.8 Cache Plane | Cached data | No | Evictable | Non-authoritative |
| L6.9 Budget Runtime | Budget enforcement | No | Mutable | Policy enforcement |

### Runtime Plane Separation

| Runtime | Responsibility | Owns Truth | Hot/Cold | Notes |
|---------|----------------|------------|----------|-------|
| L6.10 Workspace Runtime | Workspace state | No | Hot | UI-facing, non-authoritative |
| L6.11 Validation Runtime | Validation checks | No | Hot | Legality checks, non-owning |
| L6.12 Preview Runtime | Preview generation | No | Hot | Disposable, budgeted |
| L6.13 Build Runtime | Build execution | No | Cold | Deterministic, reproducible |
| L6.14 Release Runtime | Release packaging | No | Cold | Deterministic, gated |

### Sidecar Intent Separation

| Sidecar | Responsibility | Owns Truth | Hot/Cold | Notes |
|---------|----------------|------------|----------|-------|
| L6.0t Tool Session | Tool session state | No | Hot | UI-facing |
| L6.1t Tool Selection | Selection state | No | Hot | UI-facing |
| L6.2t Tool Focus | Focus state | No | Hot | UI-facing |
| L6.3t Tool Inspection | Inspection views | No | Hot | UI-facing |
| L6.4t-6.17t Tool Intents | Various intents | No | Hot | UI-facing |

## Separation Verification

### No Monolithic Mixing
- Authority core does not own data planes ✓
- Data planes do not own authority ✓
- Runtimes do not own truth ✓
- Sidecars do not own truth ✓

### Clear Boundaries
- Each plane has distinct responsibility ✓
- Each plane has clear ownership rules ✓
- Each plane has clear mutability rules ✓
- Each plane has clear hot/cold posture ✓

### No Shadow Duplication
- No plane duplicates another's truth ✓
- No plane bypasses another's authority ✓
- No plane creates hidden stores ✓

## Proof Basis

This separation is verified through:
- Plane taxonomy enumeration
- Ownership rule verification
- Boundary preservation checks
- Shadow duplication audit

## Version

This is the v4 L6 plane separation proof, active for tooling gold closure.


## Local contract mesh support
- `../../evidence/layers/tooling_level_local_contract_mesh_v4.md` records file-by-file local contract completeness for all declared levels and sidecars.
- `../../evidence/layers/tooling_family_local_contract_mesh_v4.md` records file-by-file local contract completeness for all declared families.
