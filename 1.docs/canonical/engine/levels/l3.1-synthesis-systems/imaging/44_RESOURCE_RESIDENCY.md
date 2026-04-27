# Resource Residency

## Role

Imaging-side view of runtime-owned resource residency.

## Residency Matrix

| Resource class | Canonical source | Legal use | Failure posture | Illegal posture |
|---|---|---|---|---|
| textures | runtime/resource-service residency only | read bounded resident set | degrade quality first | imaging-owned residency governor |
| geometry | runtime/resource-service residency only | read bounded resident set | cull or simplify first | hidden geometry pinning |
| frame resources | runtime-owned frame-resource policy | consume bounded frame set | drop optional pass first | persistent frame-resource leak |
| pipeline cache blobs | startup/runtime optional warm path | optional acceleration only | drop optional blobs | required pipeline-cache dependence |

## Rule

Imaging may observe residency classes and react to legal pressure, but may not become a second residency owner.

## Binding Table

| Binding | Canonical source |
|---|---|
| residency classes and windows | `STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md` |
| texture/geometry/transient ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| startup warm artifacts | `levels/l4-startup/startup/42_RUNTIME_WIRING.md` |

## Illegal Patterns

- imaging pinning resource classes permanently;
- frame-resource leak disguised as optional warmup;
- imaging-side residency debt hidden from `L1.5`.

## Failure Priority Matrix

| Pressure source | Preserve first | Degrade first |
|---|---|---|
| residency pressure | legal resident working set | optional quality tiers |
| frame-resource pressure | required transient set | optional passes |
| startup warm artifact pressure | runtime legality | optional warm blobs |

## Illegal Path Matrix

| Illegal path | Why illegal |
|---|---|
| imaging-owned residency governor | duplicates `L1.5` ownership |
| hidden persistent pinning | converts temporary need into permanent debt |
| frame-resource carryover beyond legal window | turns transient debt into leak |

## Local Operating Law

Imaging may react to residency pressure but may never redefine residency classes, hysteresis windows, or eviction order.
