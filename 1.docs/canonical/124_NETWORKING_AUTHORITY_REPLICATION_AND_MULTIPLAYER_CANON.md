# Networking Authority, Replication, Prediction, Interest, and Multiplayer Canon

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **root canonical law**.


## Purpose

This document establishes the first complete StratumX netcode canon. It raises networking from a generic packet/truth topic into a first-class engine domain with authority, replication, interest, prediction, rollback, desync recovery, replay, and editor playtest responsibilities.

## Authority model

| Mode | Owner | Use |
|---|---|---|
| Single-player authoritative local runtime | local engine | Default local simulation and editor preview. |
| Listen-host authoritative runtime | host process | Small cooperative tests and editor multiplayer preview. |
| Dedicated authoritative server | server runtime | Release-grade multiplayer posture. |
| Client-predicted view | client | Movement/input responsiveness only; cannot author truth. |
| Spectator/observer | observer client | Read-only replicated state and replay views. |

Truth mutation belongs to the authoritative runtime. Clients may propose inputs, predicted moves, interactions, and local-only presentation, but they do not directly mutate world truth.

## Replication families

| Family | Examples | Delivery posture |
|---|---|---|
| Identity | entity ids, archetypes, ownership | reliable snapshot + delta |
| Transform | position, rotation, velocity, parent space | prioritized delta, predicted for controlled actor |
| Material/world fields | wetness, heat, smoke, contamination summaries | interest-filtered deltas, compressed |
| Destruction/topology | fracture events, breach states, support changes | reliable event + retained summary |
| Projectile/combat | authoritative hit/penetration/wound results | event-driven, server authoritative |
| Living runtime | needs, schedules, squad roles, social state | interest-filtered snapshot/event blend |
| Inventory/economy | item ownership, transfer, lock state | reliable ordered |
| Audio/visual cues | non-truth presentation hints | unreliable or regenerated from truth |

## Interest management

Interest is determined by distance, visibility, audibility, squad/faction relevance, projectile hazard, quest/event relevance, editor debug pinning, and certification replay scope. Interest must be explicit and inspectable.

## Prediction and reconciliation

Client prediction is permitted for local movement, camera, input feel, animation intent, and selected non-authoritative presentation. Reconciliation must include authoritative tick id, predicted tick id, corrected state, error magnitude, smoothing policy, and editor-visible correction reason.

## Rollback and deterministic boundary

Rollback is allowed only inside declared rollback islands. The whole dream-world does not need global deterministic rollback. Netcode must declare which systems are rollback-capable, snapshot-only, event-authoritative, or presentation-regenerated.

## Desync law

A desync is a retained evidence event. It must include tick, entity set, authority owner, local view, authoritative view, divergent field list, suspected route, recovery action, and replay artifact.

## Editor multiplayer playtest

The editor must expose a multiplayer playtest lab capable of launching local server/client sets, showing replication overlays, forcing latency/loss profiles, capturing desync traces, and comparing server/client state.

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

---

# V34 netcode target and authority closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding target

StratumX 1.0 is local/single-player deterministic foundation with the data boundaries needed for future multiplayer. StratumX 1.x adds server-authoritative multiplayer foundation.

## Authority model

| Scope | Authority |
|---|---|
| local single-player | local runtime authority |
| editor multiplayer playtest | listen-server or local server authority |
| future dedicated server | server authoritative |
| local player motion | client prediction allowed, server reconciliation required |
| world simulation | server/runtime authority, not client truth |
| debris/far aftermath | retained summary/event seed replication |

## Replication model

Replicate:

- commands/intents;
- important events;
- state deltas;
- retained summaries;
- cell/interest scoped observations.

Do not replicate:

- full world state every frame;
- every far fragment;
- editor-only projections;
- unvalidated plugin state.

## Physics/network rule

Near exact objects may replicate state and correction. Far effects replicate event seeds and retained summaries. Material aftermath uses deterministic ids and summary packets.

## 1.0 acceptance

The 1.0 netcode docs are gold when future multiplayer can be added without changing engine truth ownership, world identity, asset identity, or SDK packet philosophy.
