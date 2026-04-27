# Input, Action, and Control Resolution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the route from device input to action resolution, focus/context law, camera or pawn possession, and runtime consequence.

## Scope
This canon covers:
- device events from keyboard, mouse, gamepad, or equivalent controls;
- action maps and named intents;
- context switching between editor, UI, preview, simulate, play, and debug surfaces;
- camera or pawn possession;
- latency and replay boundaries;
- publication of control diagnostics.

## Canonical route
`device event -> control binding lookup -> active context resolution -> named action or analog control output -> possession target evaluation -> runtime consequence publication`

## Control-context law
UI focus, editor camera focus, gameplay possession, cinematic control, and diagnostics capture are distinct legal contexts.
No package may hide a context switch that changes who receives action consequences.

## Possession law
Possession belongs to runtime authority.
Editor and tooling may request a legal transition into simulate or play, but they do not secretly own gameplay possession truth.

## Replay law
Action resolution is the durable unit for input consequence.
Raw device noise may be transient, but named action results and critical analog values must be publishable when required for replay, diagnostics, or deterministic runtime behavior.

## Diagnostics law
The stack must be able to explain at minimum:
- which context consumed the input;
- which action map resolved it;
- whether it was denied by focus or legality gates;
- which possession target received the consequence;
- whether a degraded or debug posture altered the route.

## Current posture
The uploaded code proves only a narrow fire-test route and broader canon-level control surfaces.
Generalized input/action/control closure remains `specified_only` or `partial` depending on the workflow.
