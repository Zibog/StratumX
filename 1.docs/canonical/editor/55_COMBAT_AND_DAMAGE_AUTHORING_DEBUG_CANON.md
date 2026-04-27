# Combat and Damage Authoring Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the product-layer workflow for combat test setup, ballistic inspection, damage inspection, and lethal-state debugging.

## Required product capabilities
The editor should eventually be able to:
- spawn or select a combatant and a target;
- assign weapon/projectile profiles;
- fire or simulate a test shot;
- inspect impact, penetration, ricochet, or stop verdicts;
- inspect actor damage and death consequences when supported;
- see diagnostics explaining deferment or degradation.

## Current implementation posture
The uploaded code proves one real narrow route:
- bootstrap a vertical-slice scene;
- fire a test shot;
- inspect impact result, damage memory, runtime events, and shot history.

The uploaded code does not yet prove general actor lethality, hit-zone consequences, or a broader combat authoring workflow.
Therefore the current posture is:
- ballistic material-impact debug -> `live_engine_backed`
- actor lethality debug -> `deferred`
