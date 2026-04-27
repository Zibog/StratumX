# L5 Bridge Export Surfaces

## Role

Startup publication of narrow public engine export surfaces for the external `L5` bridge bind.

## Export Matrix

| Export surface class | Required shape | Required guarantee | Illegal posture |
|---|---|---|---|
| packet sink | typed public ingress sink | write-side bind remains narrow and ordered | external packet write into engine internals |
| control sink | typed public ingress sink | control bind remains narrow and legal | control widening into engine execution ownership |
| observation source | immutable public batch source | read-side fanout remains shareable and bounded | mutable observation queue as public authority |
| metric source | immutable public batch source | metric fanout remains shareable and bounded | hidden mutable diagnostics authority |
| fact table | immutable public snapshot table | compatibility facts remain low-churn and typed | ad-hoc mutable fact graph |
| handle/ref table | immutable public snapshot table | handle and ref opacity remains intact | semantic widening into editor ids |
| artifact-ref table | immutable public snapshot table | artifact projections remain bounded and opaque | build ownership leak |

## Rule

`engine_startup` may publish only the named public bridge export surface classes.
Each class must remain typed, bounded, and externally consumable without exposing deep engine internals.

## Failure Posture

Illegal bridge shape, mutable deep leakage, undeclared export class, or post-launch widening fails closed before public bind publication.

## Illegal Patterns

- raw pointer or deep allocator exposure;
- external bridge bind to undeclared internal service state;
- mutable observation or metric queues as the public truth surface;
- semantic widening of handles or refs into editor authority ids.

## Local Operating Law

Public bridge export surfaces are contract surfaces only.
They exist to let `L5` bind narrowly without inheriting engine execution ownership.
