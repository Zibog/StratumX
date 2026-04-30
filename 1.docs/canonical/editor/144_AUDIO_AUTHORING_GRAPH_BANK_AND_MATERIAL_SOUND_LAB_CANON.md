# Audio Authoring Graph, Bank, Material Sound, and Audition Lab Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **editor operator lab**.


## Lab surfaces

| Surface | Controls |
|---|---|
| Event graph | event nodes, layer mixer, variation rules, state switches, debug probes. |
| Material sound matrix | surface family, event type, state, sample set, variation, fallback. |
| Bank panel | source files, memory tier, stream chunks, cook status. |
| Audition viewport | emitter/listener scenario, material underfoot, occlusion/portal/weather state. |
| Mix inspector | bus, priority, ducking, loudness, culling, device output. |

## Required recovery actions

- assign fallback sample;
- add material row;
- rebuild bank;
- fix loop/tail;
- open occlusion trace;
- inspect mix culling;
- capture audition evidence.

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
