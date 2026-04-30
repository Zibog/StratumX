# Networking Authority, Replication, Prediction, Interest, and Desync Runtime Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **engine runtime law**.


## Runtime ownership

The authoritative runtime owns replicated truth. Local clients own input prediction, camera smoothing, local presentation, and speculative animation only.

## Runtime services

| Service | Responsibility |
|---|---|
| Authority server | Tick truth, validate inputs, publish replication state. |
| Replication scheduler | Prioritize replicated families by interest and budget. |
| Interest resolver | Decide which client receives which fields and events. |
| Prediction bridge | Track predicted state and reconciliation outcomes. |
| Snapshot store | Retain authoritative snapshots for rollback islands, replay, and desync evidence. |
| Desync detector | Compare client/server views and publish retained traces. |

## Tick lifecycle

1. receive inputs;
2. validate authority and permissions;
3. apply authoritative simulation tick;
4. resolve interest per client;
5. build replication packets;
6. send prioritized deltas/events;
7. receive acknowledgements;
8. retain snapshot/evidence;
9. publish diagnostics.

## Rollback scope table

| System | Rollback posture |
|---|---|
| controlled actor movement | rollback-capable island |
| projectile hit confirmation | event-authoritative with short rewind window |
| large world weather | snapshot/delta, not full rollback |
| destruction topology | reliable event + retained summary |
| society/ecology | long-horizon authoritative summary, no client rollback |
| audio/VFX presentation | regenerated from truth, not authority |

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
