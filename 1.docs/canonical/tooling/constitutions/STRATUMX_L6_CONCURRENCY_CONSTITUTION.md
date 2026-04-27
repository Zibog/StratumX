# STRATUMX_L6_CONCURRENCY_CONSTITUTION

## Scope
This constitution governs concurrency posture inside tooling.

## Binding laws
- concurrency must be expressed through explicit queues, cursors, epochs, transactions, or immutable snapshots;
- background work is bounded and externally visible;
- no service may hide critical state in unsurfaced threads or caches.

## Audit checks
- `31_THREADING.md` docs remain concrete and level-specific;
- build/preview/validation/background services expose bounded concurrency semantics;
- editor-facing consumers can reason about freshness and ordering.
