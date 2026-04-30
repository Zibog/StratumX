# Netcode Playtest, Replication, Desync, and Replay Routing Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **tooling route canon**.


## Routes

| Route | Purpose |
|---|---|
| `route.net.playtest.start` | Launch local server/client playtest. |
| `route.net.playtest.stop` | Stop playtest and retain evidence. |
| `route.net.latency_profile.apply` | Apply latency/loss/jitter profile. |
| `route.net.replication.inspect` | Inspect replicated families and interest reasons. |
| `route.net.desync.capture` | Capture divergence evidence. |
| `route.net.replay.open` | Open retained network replay. |
| `route.net.authority.explain` | Explain who owns truth and why. |

## Blockers

- server bootstrap failed;
- client connect failed;
- authority mismatch;
- unsupported rollback scope;
- replication family missing;
- desync trace incomplete;
- replay artifact unavailable.

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
