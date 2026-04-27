# Snapshot, Index, Derived, Artifact Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Snapshots are immutable, versioned, and swap-safe.
Indices are rebuildable, scoped, and replaceable.
Derived projections are disposable and never authoritative.
Artifacts are deterministic, manifest-backed, and invalidation-driven.
Caches are bounded, evictable, and non-authoritative.

## Required answers for every heavy data class
- where does authority live?
- which snapshot class carries the stable read view?
- which indices accelerate it?
- which derived projections are disposable convenience only?
- which artifacts are deterministic build products?
- which caches are evictable?
- which invalidation roots rebuild which products?

Any class that cannot answer whether it is mutable, immutable, rebuildable, deterministic, disposable, or evictable is not ready for this stack.
