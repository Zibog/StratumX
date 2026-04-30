# Audio Authoring, Bank, Event, Material Sound, and Mix Packet Catalog

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **SDK packet catalog**.


## Packet families

| Packet | Purpose |
|---|---|
| `audio.event.definition.v1` | Authored event graph summary and event identity. |
| `audio.bank.manifest.v1` | Cooked bank identity, stream chunks, memory tier. |
| `audio.material.row.v1` | Material/surface event variant row. |
| `audio.audition.request.v1` | Editor audition request with parameters and scenario. |
| `audio.audition.result.v1` | Rendered/auditioned result metadata and blockers. |
| `audio.mix.diagnostic.v1` | Bus, priority, ducking, culling, loudness diagnostic. |
| `audio.occlusion.trace.v1` | Emitter/listener/portal/zone path evidence. |
| `audio.failure.detail.v1` | Missing event/bank/sample/route/row failure. |

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
