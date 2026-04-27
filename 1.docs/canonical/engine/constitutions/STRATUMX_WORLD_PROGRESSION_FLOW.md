# STRATUMX_WORLD_PROGRESSION_FLOW

## 1. Purpose

This document defines the canonical progression flow of StratumX from startup through runtime execution into authoritative world progression.

It is the sequencing map built on top of the execution constitution. It does not redefine execution law.

## 2. Canonical Flow

```text
engine_startup
    -> startup-ready assembly decisions
    -> runtime profile selection
    -> network role selection
    -> runtime launch

runtime family
    -> ingress
    -> read
    -> compute
    -> resource
    -> authority-sync
    -> stage
    -> apply
    -> egress
    -> diagnostics

runtime resource services
    -> activation / prefetch / eviction
    -> residency / pressure
    -> allocators / pools / lifetimes
    -> decode / staging / upload

critical simulation families
    -> local kinetics products
    -> field products
    -> agent products

network runtime services
    -> transport products
    -> interest / snapshot / delta / ack products
    -> prediction / reconciliation / rewind products

service layers
    -> model products
    -> synthesis products
    -> resource products

engine_world
    -> authoritative progression
```

## 3. Startup Stage

`engine_startup`:

- loads canonical configuration;
- produces startup-ready assembly decisions;
- selects a legal runtime profile;
- selects a legal network role when networking is enabled;
- wires legal families, runtime resource services, network runtime services, and service layers;
- launches the runtime family.

## 4. Runtime Stage

The active runtime authority:

- opens legal read windows;
- schedules critical simulation families;
- schedules runtime resource services where budgets or cadence require them;
- schedules legal network runtime services where role and cadence require them;
- schedules legal service-layer work when profile and cadence require it;
- collects staged outputs;
- orders apply;
- progresses world truth;
- emits events, outputs, and diagnostics.

## 5. Canonical World Progression Law

The world progresses only through runtime-owned apply, even when resource services and network services are active.
