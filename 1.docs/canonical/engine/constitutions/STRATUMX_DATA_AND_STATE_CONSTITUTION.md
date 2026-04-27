# STRATUMX_DATA_AND_STATE_CONSTITUTION

## 1. Purpose

This document defines the canonical data and state law of StratumX.

## 2. Data Classes

### 2.1. Authoritative state
Owned by `engine_world`. This is the only authoritative world truth.

### 2.2. Shared world property state
Owned by `engine_material`. Read-heavy substrate consumed by upper families and service layers.

### 2.3. Snapshot state
Immutable read state produced from authoritative world truth.

### 2.4. Read views
Localized legal read shapes built from snapshots.

### 2.5. Staged state
Family-produced deltas and service outputs awaiting canonical apply or downstream use.

### 2.6. Derived state
Non-authoritative state derived from authoritative truth.

### 2.7. Transient execution state
Runtime-owned execution, scheduling, diagnostics, and staging structures that are not persisted as world truth.

### 2.8. Runtime resource state
Resource-service-owned transient state for streaming queues, residency maps, allocators, staging blocks, and transfer fences.

### 2.9. Network transient state
Network-service-owned transient state for transport buffers, snapshot sets, ack sets, prediction buffers, and rewind history.

### 2.10. Restoration state
State required to reconstruct a legal authoritative engine assembly from a persisted payload.

## 3. Canonical Restoration Terms

Canonical shapes are defined by:
- `STRATUMX_CANONICAL_SHAPES.md`

### 3.1. Runtime-profile-safe restoration state
`RuntimeProfileSafeRestorationState` is the minimal legal restoration package.

### 3.2. Profile-safe restoration selectors
`ProfileSafeRestorationSelectors` choose the legal runtime profile, restoration scope, and payload interpretation without restoring transient runtime internals.

## 4. State Ownership Laws

- only `engine_world` owns authoritative world truth;
- only `engine_material` owns shared world property substrate;
- runtime owns transient execution state, not world truth;
- runtime resource services own transient streaming/residency/memory/transfer state, not world truth;
- network runtime services own transient transport/sync/latency state, not world truth;
- families own staged outputs until apply or discard;
- service layers own service-local results, not world truth.

## 5. Canonical Laws

- no direct authoritative mutation from family or service layers;
- no persistence of transient runtime internals, transient resource-service internals, or transient network-service internals as authoritative restoration state;
- no restoration without profile-safe restoration selectors;
- no restoration package may omit the canonical stack version marker.
