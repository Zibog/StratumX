# Asset/Model Import, Cook, LOD, Proxy, and Failure Routing Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **tooling route canon**.


## Routes

| Route | Purpose |
|---|---|
| `route.asset.import` | Parse and normalize source asset. |
| `route.asset.validate` | Validate units, axes, materials, skeleton, collision, texture channels. |
| `route.asset.preview` | Build preview artifact. |
| `route.asset.generate_collision` | Generate/validate collision. |
| `route.asset.generate_lod_proxy` | Generate LOD/proxy/impostor. |
| `route.asset.bind_runtime` | Produce runtime binding package. |
| `route.asset.cook` | Produce cooked package. |
| `route.asset.explain_failure` | Return exact recovery path. |

## Artifact law

Every route produces an artifact record even on failure. Failed imports retain source identity, parser verdict, normalized metadata if available, first blocker, and recovery hint.

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
