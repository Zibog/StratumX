# Degradation Policy Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines how tooling degrades under pressure without violating correctness.

## Golden rule
Degrade presentation and speculation first.
Never degrade authority, transaction correctness, required invalidation, or artifact determinism.

## Allowed degradation order
1. secondary thumbnails and non-focused previews
2. speculative derived summaries
3. dormant suite caches
4. wide-scope background indexing in favor of focused-scope indexing
5. low-priority validation scans
6. low-priority background reimport/bake queues
7. non-critical workspace coordination caches

## Forbidden degradation
- dropping transaction records
- skipping required invalidation propagation
- hiding failed build/release jobs
- returning stale prefab override truth as if current
- masking cross-layer reference errors to keep UI smooth
- keeping stale runtime-bridge bindings after authority changed
- capturing product UI state inside tooling to save refresh cost

## User-visible rule
Every degraded mode must surface:
- what degraded,
- why it degraded,
- what stayed correct,
- how to recover full fidelity.
