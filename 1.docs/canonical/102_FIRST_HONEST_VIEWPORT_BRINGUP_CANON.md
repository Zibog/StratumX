# First Honest Viewport Bringup Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

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
