# Audio Authoring Graph, Bank, Material Sound, and Occlusion Runtime Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **engine runtime law**.


## Runtime audio graph execution

The runtime executes cooked audio graphs as event plans. It does not interpret editor-only graph layout; it consumes normalized nodes, event layers, variation rules, routing, and fallback rows.

## Runtime lifecycle

1. receive audio event intent;
2. resolve emitter/listener;
3. resolve material/state variant;
4. evaluate graph parameters;
5. resolve bank residency;
6. apply occlusion/portal/zone influence;
7. schedule layers on mix buses;
8. publish timing and diagnostic packets;
9. retain audition/certification evidence when requested.

## Required runtime diagnostics

- event not found;
- bank missing;
- sample missing;
- no material sound row;
- occlusion portal invalid;
- bus unavailable;
- priority culled;
- streaming late;
- device unavailable.

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
