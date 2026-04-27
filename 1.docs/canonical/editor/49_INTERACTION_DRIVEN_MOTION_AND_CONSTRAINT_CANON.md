# Interaction-Driven Motion and Constraint Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the next-wave canonical posture for natural character interaction with world objects.
It exists to prevent “door handles by chaos”, “full simulation too early”, and animation/ik/contact logic being scattered across unrelated systems.

This canon is not the blocker for the first terrain + sky + walk closure.
It is the first major maturity wave after the world-authoring spine is honest.

## Product formula
StratumX interaction-driven motion is:
`Authored Motion + Runtime Adaptation + Contact Constraints + Micro Motion + Optional Physics Assist`.

## Non-goal
This canon does not authorize replacing the first honest product spine with a complex animation showcase.
The editor must first be able to open a world, show terrain and environment, enter play/walk, and return cleanly.
Only then may this system become hot.

## Problem statement
World interaction such as opening a door, pulling a lever, or pressing a button is not a single animation file.
It is a layered execution problem:
1. intent exists
2. a strategy is selected
3. a base motion is chosen
4. pose is adapted to the real target
5. contact is stabilized
6. micro-motion adds life

## Layer ownership map
### Engine
- interaction target truth
- animation execution order
- runtime adaptation
- ik solving
- contact constraint solving
- optional secondary motion or physics assist

### SDK
- interaction refs
- animation/interaction observations
- result-bearing runtime/debug packets

### Tooling
- authoring requests
- preview/runtime coordination
- debug projection aggregation
- validation and diagnostics for targets, profiles, and constraints

### Editor
- authoring surfaces for interaction targets and profiles
- preview controls
- debug overlays
- inspector exposure
- no hidden animation/runtime truth

## Core data families
### Intent
- `InteractionIntent`
- `InteractionTargetRef`
- `InteractionProfileRef`

### Strategy / execution
- `ExecutionPolicy`
- `PreferredHand`
- `ContactMode`
- `ManipulationMode`

### Motion assets
- `MotionAssetRef`
- `MotionPhase`
- `MotionEvent`

### Runtime adaptation
- `ReachTarget`
- `PoleTarget`
- `ChestCompensation`
- `RootAlignmentPolicy`

### Contact constraints
- `GripConstraint`
- `OrientationConstraint`
- `ContactLockState`

### Polish / debug
- `MicroMotionPolicy`
- `SecondaryMotionPolicy`
- `InteractionDebugProjection`

## Execution law
The runtime execution order must remain explicit:
1. base locomotion or authored action pose
2. clip/blend selection
3. additive layers
4. runtime adaptation and warping
5. ik solve
6. contact lock and constraint stabilization
7. micro-motion and secondary motion
8. final skeleton output

If this order is mixed arbitrarily, the result is invalid.

## Strategy law
One interaction may have multiple legal strategies:
- authored only
- authored + light correction
- authored + meaningful procedural adaptation
- procedural-first with authored support

The engine must choose strategy explicitly, not implicitly by animation asset accident.

## Contact law
Targets such as handles, levers, or buttons must expose explicit interaction truth:
- target transform
- grip orientation
- preferred hand
- contact windows
- manipulation mode

A runtime may not invent contact semantics ad hoc from raw mesh transforms.

## Editor surface law
The future animation/interactions authoring surface must expose:
- interaction target placement
- profile assignment
- phase markers and event timing
- ik target preview
- contact debug overlays
- execution-policy selection
- diagnostics for invalid reach, invalid constraints, or bad phase timing

## Diagnostics law
The product must be able to explain at least:
- missing target
- invalid profile
- invalid phase order
- ik solve failure
- contact lock failure
- runtime adaptation clamp
- unsupported body/rig posture

## Implementation order
This system activates only after the first honest world spine.
The canonical order is:
1. open world, terrain, environment, walk, return
2. add interaction target truth
3. add authored motion with phase markers
4. add runtime adaptation and basic IK
5. add contact constraints
6. add micro-motion
7. add optional physics assist
8. expose full diagnostics and editor authoring tools

## Acceptance note
This canon is satisfied when interaction-driven motion is layered on top of the honest editor/runtime spine without creating a second hidden runtime, a second hidden animation truth, or a physics-first chaos system.
