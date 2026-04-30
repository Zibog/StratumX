# Asset/Model Import, Binding, Content, and Failure Packet Catalog

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **SDK packet catalog**.


## Packet families

| Packet | Purpose |
|---|---|
| `asset.import.request.v1` | Request import/canonicalization of a source asset. |
| `asset.import.verdict.v1` | Success, warning, failure, partial preview verdict. |
| `asset.normalized.metadata.v1` | Units, axes, scale, color space, material slots, skeleton info. |
| `asset.runtime.binding.v1` | Runtime binding handles and fallback refs. |
| `asset.preview.artifact.v1` | Preview thumbnail/mesh/material/collision evidence. |
| `asset.cook.result.v1` | Cooked package identity, warnings, blockers. |
| `asset.failure.detail.v1` | Exact import/cook/bind failure. |

## Consumer table

| Consumer | Required packets |
|---|---|
| content browser | import verdict, preview artifact, runtime binding status |
| editor inspector | normalized metadata, material slots, collision, LOD/proxy |
| tooling cook | import request, cook result, failure detail |
| engine runtime | runtime binding, fallback refs, residency class |

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
