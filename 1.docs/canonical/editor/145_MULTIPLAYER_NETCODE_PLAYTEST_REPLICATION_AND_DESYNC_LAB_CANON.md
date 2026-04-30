# Multiplayer Netcode Playtest, Replication, Prediction, and Desync Lab Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **editor operator lab**.


## Lab surfaces

| Surface | Controls |
|---|---|
| Session setup | server/client count, map, authority mode, latency/loss profile. |
| Replication inspector | replicated families, interest reason, update rate, bandwidth, blockers. |
| Prediction view | predicted state, authoritative correction, smoothing, error magnitude. |
| Desync trace | divergent field list, tick, entity set, authority owner, replay artifact. |
| Replay panel | load, scrub, compare server/client views, export evidence. |

## Required overlays

- authority owner badge;
- replicated/not replicated reason;
- prediction correction vectors;
- bandwidth pressure;
- desync hazard marker;
- rollback island boundary.

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
