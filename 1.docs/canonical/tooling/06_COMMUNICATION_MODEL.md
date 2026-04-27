# Communication Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Canonical exchange classes:
- command envelopes
- binding controls
- execution signals
- transaction admissions
- transaction results
- invalidation sets
- immutable snapshots
- rebuildable indices
- derived projections
- deterministic artifacts and artifact manifests
- bounded streams
- bounded caches
- context evidence packs
- proposals
- plan bundles
- campaign bundles
- policy bundles
- pressure frames
- legality and safety verdicts
- opaque handles and opaque refs

Communication law:
- all exchange is typed
- all hot and warm-path exchange is bounded
- all mutation exchange is replayable or auditable
- all ownership transfer is explicit
- all expensive exchange classes must expose budget class and lifetime class
- object soup, ad-hoc maps, and hidden shared mutable blobs are forbidden
