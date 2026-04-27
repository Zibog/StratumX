# Query Descriptors

## Role

Canonical descriptor surface for compiled ECS query plans.

## Descriptor Matrix

| Field | Canonical law | Invalidation law | Illegal posture |
|---|---|---|---|
| component set | explicit and closed | compile invalid only on structural mismatch | inferred on bind |
| access mode | explicit read/write legality | invalid on legality widening | caller-convention legality |
| locality class | explicit and frozen | invalid on locality change only | implicit widening |
| partitionability | explicit | invalid on partition-class change | per-iteration repartitioning |
| cache key | explicit and stable | invalid on descriptor identity change | hidden per-call mutation |

## Rule

A cache-hit descriptor may not compile a new plan, widen access, or mutate partitionability.

## Binding Table

| Binding | Canonical source |
|---|---|
| traversal legality | `STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md` |
| numeric shared constants | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |

## Compiled-Legality Matrix

| Descriptor field | Must be known at compile/bind time | Illegal posture |
|---|---|---|
| component set | yes | inferred on bind |
| access mode | yes | caller-convention legality |
| locality class | yes | hidden widening |
| partitionability | yes | runtime repartitioning by surprise |
| cache identity | yes | hidden per-call mutation |

## Local Operating Law

Query descriptors are compiled-legality surfaces.
They exist to prevent bind-time improvisation, hidden access widening, and cache-identity drift.
