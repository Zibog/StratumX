# Traversal Entry

## Role

Entry surface for compiled traversal.

## Entry Matrix

| Concern | Canonical law | Runtime assertion | Illegal posture |
|---|---|---|---|
| plan source | compiled legal traversal plan only | plan id must match descriptor/cache entry | ad hoc bind-time compile on cache hit |
| access legality | explicit in type or descriptor | assert read/write legality at bind | caller-convention legality |
| locality class | frozen at bind | assert locality class match | hidden region widening |
| scratch class | frozen at bind | assert owning scratch class | hidden pool swap after bind |
| mutation handoff | staged only | assert no direct write entry path | direct write in traversal entry |

## Rule

Traversal entry may not allocate a new plan on cache hit, widen locality or access rights after bind, or rewrite partition boundaries during iteration.

## Binding Table

| Binding | Canonical source |
|---|---|
| traversal legality | `STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md` |
| scratch/queue ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |

## Traversal Entry Matrix

| Entry concern | Required seal | Illegal posture |
|---|---|---|
| compiled plan source | mandatory | ad hoc bind-time compile |
| access rights | sealed at bind | hidden write widening |
| locality class | sealed at bind | hidden region widening |
| scratch ownership | sealed at bind | pool swap after bind |
| mutation handoff | staged only | direct write entry |

## Local Operating Law

Traversal entry is a legality checkpoint, not a convenience helper.
It may not normalize plan creation, access widening, or locality drift after bind.
