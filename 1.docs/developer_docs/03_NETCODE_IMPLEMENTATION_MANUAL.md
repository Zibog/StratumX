# Netcode Implementation Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer guide**.


## Goal

Implement enough networking to reach the V33 75% document-readiness target without pretending to ship a mature multiplayer stack.

## Implementation order

1. authority posture service;
2. tick identity and snapshot ids;
3. input intent packet;
4. replication family registry;
5. interest resolver;
6. transform replication;
7. controlled actor prediction/reconciliation;
8. desync detector;
9. local multiplayer playtest launcher;
10. replay/capture evidence.

## Do not implement yet

Do not implement global rollback for the entire dream world. Implement rollback islands and retained summaries.

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
