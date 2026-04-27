# Budget Runtime Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Budget classes
- authority budget
- snapshot/index rebuild budget
- validation budget
- preview budget
- build budget
- release budget
- cache budget
- stream/diagnostics budget
- workspace-coordination budget

## Priority law
The tooling stack protects budgets in this order:
1. authority commit latency;
2. transaction and snapshot swap latency;
3. visible editor interaction latency;
4. focused validation turnaround;
5. runtime-attach and focused preview correctness;
6. background import/reimport/bake/build throughput;
7. release throughput;
8. speculative preview and cache richness.

## Canonical posture
- authority commits must stay tiny and predictable;
- index rebuilds must be scoped by invalidation set whenever possible;
- validation must support focused, scope-bounded reruns;
- preview work must be cancellable;
- build and bake queues must be classed by budget and pressure visibility;
- release jobs must be cold and non-interfering with focused editing;
- workspace runtime may spend budget on leases/cursors/session binds only, never on product widget trees.

## Suggested envelopes
- focused authority commit: sub-frame target
- focused index/derived refresh: sub-100ms target for small scopes
- focused validation rerun: sub-250ms target for local scope
- preview warmup: sub-second target for ordinary asset/entity previews
- background reimport/bake/build: queue-scheduled and progress-visible
- release/package jobs: explicitly deprioritized relative to active authoring

The exact numbers may vary by project, but the class ordering may not.
