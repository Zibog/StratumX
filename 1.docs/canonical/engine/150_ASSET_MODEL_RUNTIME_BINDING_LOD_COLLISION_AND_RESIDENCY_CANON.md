# Asset/Model Runtime Binding, LOD, Collision, Proxy, and Residency Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **engine runtime law**.


## Runtime binding contract

Imported assets are not runtime-ready until they produce binding packages with mesh sections, material slots, bounds, collision refs, residency class, streaming identity, and fallback identity.

## Binding families

| Family | Runtime binding data |
|---|---|
| Static mesh | sections, vertex/index buffers, material slots, bounds, collision handle, LOD chain. |
| Skeletal mesh | skeleton handle, skin buffers, sections, material slots, animation compatibility. |
| Terrain tile | height/mesh data, material layer handles, collision field, streaming cell. |
| Foliage | instance data, wind class, impostor tier, material refs. |
| Groom/fur | coverage buffers, near/far tier policy, material/wetness/wind channels. |
| Collision | broadphase proxy, narrowphase shape, traversal class, destruction link. |

## Residency

Every asset reports required memory tier, streaming priority, fallback proxy, missing-resource behavior, and editor-visible pressure diagnostics.

## LOD/proxy law

LOD is not optional for large-world content. Every large-world asset family must define at least one fallback representation: simplified mesh, impostor, retained summary, or missing placeholder.

## Non-negotiable rules

| Rule | Meaning |
|---|---|
| No fake success | A route may return `NotImplemented`, `Blocked`, `Unavailable`, or `Unsupported`, but it may not return success for an unimplemented behavior. |
| One owner | Every truth object has exactly one owner layer. Other layers may hold handles, DTOs, views, or cached projections only. |
| Observable failure | Every failure family must produce an error code, disabled reason, recovery hint, and retained diagnostic packet. |
| Editor honesty | The editor may expose the route, preview, capture, or recovery action, but it must not mutate engine truth except through lawful SDK/tooling ingress. |
| Evidence or it did not happen | Release-grade claims require capture, compare, replay, or retained diagnostic evidence. |

## Required completion shape

Every implementation derived from this document must include:

1. owner module or crate;
2. public data contracts;
3. lifecycle stages;
4. failure and disabled reason codes;
5. editor/tooling/SDK contact points;
6. quality tests and negative-path tests;
7. retained evidence artifacts for certification routes.
