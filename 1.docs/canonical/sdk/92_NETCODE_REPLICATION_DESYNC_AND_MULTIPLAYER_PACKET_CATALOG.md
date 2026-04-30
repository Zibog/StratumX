# Netcode Replication, Desync, and Multiplayer Packet Catalog

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **SDK packet catalog**.


## Packet families

| Packet | Direction | Purpose |
|---|---|---|
| `net.authority.posture.v1` | engine → tooling/editor | Current authority mode and owner. |
| `net.input.intent.v1` | client/editor → authority | Client input proposal. |
| `net.replication.delta.v1` | authority → client/editor | Field/event delta with family and interest reason. |
| `net.snapshot.summary.v1` | authority → tooling/editor | Snapshot identity, tick, retained fields. |
| `net.prediction.reconcile.v1` | authority → client/editor | Correction data and smoothing policy. |
| `net.interest.explain.v1` | authority/tooling → editor | Why object is/is not replicated. |
| `net.desync.trace.v1` | engine/tooling → editor/evidence | Divergence evidence. |
| `net.playtest.session.v1` | editor/tooling → engine | Local multiplayer playtest setup. |

## Required fields

All network packets include `session_id`, `tick`, `authority_owner`, `scope`, `family`, `version`, and `diagnostic_context`.

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
