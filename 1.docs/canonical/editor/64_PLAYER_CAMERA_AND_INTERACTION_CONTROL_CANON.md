# Player, Camera, and Interaction Control Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines gameplay-facing control asks at the editor/product layer.
It covers movement, look, interact, fire, locomotion modifiers, possession, and camera mode switching.

## Canonical route
`named action or analog control -> active possession target -> movement/look/interaction consequence -> runtime observation and diagnostics -> editor/runtime presentation`

## Required action classes
| Action class | Examples |
|---|---|
| locomotion_action | move, sprint, crouch, jump, prone, lean |
| camera_action | look, freelook, orbit, recenter, zoom/FOV adjustments where legal |
| interaction_action | use/open/pick up/push/activate |
| combat_action | fire, aim, reload, swap, inspect weapon |
| state_transition_action | enter simulate, enter play, release possession, return to editor |

## Possession law
The product layer must expose whether input currently targets:
- editor navigation camera;
- gameplay pawn;
- cinematic camera;
- UI-only focus;
- diagnostics-only consumer.

## Disabled-state law
Dead, downed, stunned, menu-captured, or legality-blocked states must explain why an action did not produce the expected consequence.

## Current posture
A narrow fire-test path already exists in code.
This document generalizes the route canonically so the stack can answer broader gameplay-control asks.
