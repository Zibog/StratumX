# First Honest Viewport Bringup Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Freeze the minimum legal bring-up sequence for the first honest editor viewport.

This file exists so implementation stops wandering between decorative shells, demo viewports, and fake frame paths.

## First-result signature
The first honest bring-up result is:

`editor host booted -> project opened -> world opened -> terrain visible -> sky/environment visible -> one legal camera -> one legal diagnostics rail -> same lawful frame chain used for future capture`

## Cold-start ladder
1. bootstrap native window
2. bootstrap device and present surface
3. publish backend class and feature verdicts
4. mount shell frame and layout
5. restore or open project
6. restore or open world
7. resolve primary camera and viewset
8. resolve terrain/geometry visibility
9. resolve sky, atmosphere, and light bindings
10. submit first lawful frame
11. paint diagnostics badges in shell
12. enable stage strip, inspector, outliner, and content browser

## Failure posture
If any bring-up step fails:
- the shell stays open;
- the viewport region stays visible;
- the failure is rendered in the viewport shell and diagnostics rail;
- the active backend class and missing boundary are published.

## Honest viewport law
The first viewport may be low richness.
It may not be fake.
Specifically forbidden:
- static splash image as primary viewport;
- decorative sky card covering missing world-open;
- UI mock preview unrelated to runtime frame chain;
- editor-only material preview pretending to be world render.

## Bring-up capture law
The archive must be able to capture:
- cold boot failure;
- first successful terrain+sky frame;
- recovered run after one blocker fix.

## Minimum operator controls
The first honest viewport is not complete until the shell can legally perform:
- orbit / pan / dolly;
- select object or terrain target;
- switch between world, simulation, and capture stages;
- inspect backend and degrade posture;
- save layout and recover to last good shell state.

## Required companion docs
- root `42`
- engine `77`, `86–92`, `103–107`
- editor `45`, `48`, `114`, `115`, `117`
- sdk `64`, `78`
- tooling `67`, `82`


## v31 graphics-port bring-up clarification
The first honest viewport must bring up through StratumX Native Graphics Port.

Minimum backend sequence:
1. resolve backend policy;
2. publish selected backend and fallback chain;
3. use null backend for headless validation;
4. use Vulkan only as first real backend, not as editor truth;
5. show DX12/Metal/platform-native stubs as truthful reserved slots;
6. publish backend caps before first frame;
7. publish black-frame reason if present fails.

The first honest viewport is complete only when the operator can see backend class, policy, feature tier, present path, capture readiness, and first blocker.


---
# V32 Honest Viewport Completion Rules

## Bring-up sequence
1. null frame-plan validation;
2. backend doctor visible in editor;
3. real backend present path selected by policy;
4. clear/present with backend metadata;
5. named framegraph diagnostics;
6. terrain/proof mesh visible;
7. sky/background visible;
8. material visual baseline active;
9. exposure/tonemap active;
10. capture artifact retained;
11. black-frame failure can be forced and explained.

## Viewport badges
Viewport must display or link to:
- backend class;
- backend status;
- selected policy;
- feature tier;
- shader target;
- present state;
- surface extent;
- frame-in-flight state;
- fallback rung;
- capture readiness;
- first blocker;
- validation warning count when available.

## Forbidden shortcuts
A viewport is not honest if it uses a separate editor-only renderer, a UI-only texture without frame metadata, or native backend handles in editor code.
